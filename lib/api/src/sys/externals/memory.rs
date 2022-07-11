use crate::sys::context::{AsContextMut, AsContextRef};
use crate::sys::exports::{ExportError, Exportable};
use crate::sys::externals::Extern;
use crate::sys::MemoryType;
use crate::MemoryAccessError;
use std::convert::TryInto;
use std::marker::PhantomData;
use std::mem;
use std::mem::MaybeUninit;
use std::slice;
use wasmer_types::Pages;
use wasmer_vm::{
    ContextHandle, ContextObjects, InternalContextHandle, MemoryError, VMExtern, VMMemory,
};

/// A WebAssembly `memory` instance.
///
/// A memory instance is the runtime representation of a linear memory.
/// It consists of a vector of bytes and an optional maximum size.
///
/// The length of the vector always is a multiple of the WebAssembly
/// page size, which is defined to be the constant 65536 – abbreviated 64Ki.
/// Like in a memory type, the maximum size in a memory instance is
/// given in units of this page size.
///
/// A memory created by the host or in WebAssembly code will be accessible and
/// mutable from both host and WebAssembly.
///
/// Spec: <https://webassembly.github.io/spec/core/exec/runtime.html#memory-instances>
#[derive(Debug, Clone)]
pub struct Memory {
    handle: ContextHandle<VMMemory>,
}

impl Memory {
    /// Creates a new host `Memory` from the provided [`MemoryType`].
    ///
    /// This function will construct the `Memory` using the store
    /// [`BaseTunables`][crate::sys::BaseTunables].
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value};
    /// # use wasmer::Context as WasmerContext;
    /// # let store = Store::default();
    /// # let mut ctx = WasmerContext::new(&store, (), ());
    /// #
    /// let m = Memory::new(&mut ctx, MemoryType::new(1, None, false)).unwrap();
    /// ```
    pub fn new(ctx: &mut impl AsContextMut, ty: MemoryType) -> Result<Self, MemoryError> {
        let mut ctx = ctx.as_context_mut();
        let tunables = ctx.store().tunables();
        let style = tunables.memory_style(&ty);
        let memory = tunables.create_host_memory(&ty, &style)?;

        Ok(Self {
            handle: ContextHandle::new(ctx.objects_mut(), memory),
        })
    }

    /// Returns the [`MemoryType`] of the `Memory`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value};
    /// # use wasmer::Context as WasmerContext;
    /// # let store = Store::default();
    /// # let mut ctx = WasmerContext::new(&store, (), ());
    /// #
    /// let mt = MemoryType::new(1, None, false);
    /// let m = Memory::new(&mut ctx, mt).unwrap();
    ///
    /// assert_eq!(m.ty(&mut ctx), mt);
    /// ```
    pub fn ty(&self, ctx: &impl AsContextRef) -> MemoryType {
        self.handle.get(ctx.as_context_ref().objects()).ty()
    }

    /// Returns the pointer to the raw bytes of the `Memory`.
    //
    // This used by wasmer-emscripten and wasmer-c-api, but should be treated
    // as deprecated and not used in future code.
    #[doc(hidden)]
    pub fn data_ptr(&self, ctx: &impl AsContextRef) -> *mut u8 {
        self.buffer(ctx).base
    }

    /// Returns the size (in bytes) of the `Memory`.
    pub fn data_size(&self, ctx: &impl AsContextRef) -> u64 {
        self.buffer(ctx).len.try_into().unwrap()
    }

    /// Returns the size (in [`Pages`]) of the `Memory`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value};
    /// # use wasmer::Context as WasmerContext;
    /// # let store = Store::default();
    /// # let mut ctx = WasmerContext::new(&store, (), ());
    /// #
    /// let m = Memory::new(&mut ctx, MemoryType::new(1, None, false)).unwrap();
    ///
    /// assert_eq!(m.size(&mut ctx), Pages(1));
    /// ```
    pub fn size(&self, ctx: &impl AsContextRef) -> Pages {
        self.handle.get(ctx.as_context_ref().objects()).size()
    }

    /// Grow memory by the specified amount of WebAssembly [`Pages`] and return
    /// the previous memory size.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value, WASM_MAX_PAGES};
    /// # use wasmer::Context as WasmerContext;
    /// # let store = Store::default();
    /// # let mut ctx = WasmerContext::new(&store, (), ());
    /// #
    /// let m = Memory::new(&mut ctx, MemoryType::new(1, Some(3), false)).unwrap();
    /// let p = m.grow(&mut ctx, 2).unwrap();
    ///
    /// assert_eq!(p, Pages(1));
    /// assert_eq!(m.size(&mut ctx), Pages(3));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if memory can't be grown by the specified amount
    /// of pages.
    ///
    /// ```should_panic
    /// # use wasmer::{Memory, MemoryType, Pages, Store, Type, Value, WASM_MAX_PAGES};
    /// # use wasmer::Context as WasmerContext;
    /// # let store = Store::default();
    /// # let mut ctx = WasmerContext::new(&store, (), ());
    /// #
    /// let m = Memory::new(&mut ctx, MemoryType::new(1, Some(1), false)).unwrap();
    ///
    /// // This results in an error: `MemoryError::CouldNotGrow`.
    /// let s = m.grow(&mut ctx, 1).unwrap();
    /// ```
    pub fn grow<IntoPages>(
        &self,
        ctx: &mut impl AsContextMut,
        delta: IntoPages,
    ) -> Result<Pages, MemoryError>
    where
        IntoPages: Into<Pages>,
    {
        self.handle
            .get_mut(ctx.as_context_mut().objects_mut())
            .grow(delta.into())
    }

    /// Safely reads bytes from the memory at the given offset.
    ///
    /// The full buffer will be filled, otherwise a `MemoryAccessError` is returned
    /// to indicate an out-of-bounds access.
    ///
    /// This method is guaranteed to be safe (from the host side) in the face of
    /// concurrent writes.
    pub fn read(
        &self,
        ctx: &impl AsContextRef,
        offset: u64,
        buf: &mut [u8],
    ) -> Result<(), MemoryAccessError> {
        self.buffer(ctx).read(offset, buf)
    }

    /// Safely reads bytes from the memory at the given offset.
    ///
    /// This method is similar to `read` but allows reading into an
    /// uninitialized buffer. An initialized view of the buffer is returned.
    ///
    /// The full buffer will be filled, otherwise a `MemoryAccessError` is returned
    /// to indicate an out-of-bounds access.
    ///
    /// This method is guaranteed to be safe (from the host side) in the face of
    /// concurrent writes.
    pub fn read_uninit<'a>(
        &self,
        ctx: &impl AsContextRef,
        offset: u64,
        buf: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut [u8], MemoryAccessError> {
        self.buffer(ctx).read_uninit(offset, buf)
    }

    /// Safely writes bytes to the memory at the given offset.
    ///
    /// If the write exceeds the bounds of the memory then a `MemoryAccessError` is
    /// returned.
    ///
    /// This method is guaranteed to be safe (from the host side) in the face of
    /// concurrent reads/writes.
    pub fn write(
        &self,
        ctx: &impl AsContextRef,
        offset: u64,
        data: &[u8],
    ) -> Result<(), MemoryAccessError> {
        self.buffer(ctx).write(offset, data)
    }

    pub(crate) fn buffer<'a>(&'a self, ctx: &'a impl AsContextRef) -> MemoryBuffer<'a> {
        let definition = self.handle.get(ctx.as_context_ref().objects()).vmmemory();
        let def = unsafe { definition.as_ref() };
        MemoryBuffer {
            base: def.base,
            len: def.current_length,
            marker: PhantomData,
        }
    }

    pub(crate) fn from_vm_extern(
        ctx: &impl AsContextRef,
        internal: InternalContextHandle<VMMemory>,
    ) -> Self {
        Self {
            handle: unsafe {
                ContextHandle::from_internal(ctx.as_context_ref().objects().id(), internal)
            },
        }
    }

    /// Checks whether this `Memory` can be used with the given context.
    pub fn is_from_context(&self, ctx: &impl AsContextRef) -> bool {
        self.handle.context_id() == ctx.as_context_ref().objects().id()
    }

    pub(crate) fn to_vm_extern(&self) -> VMExtern {
        VMExtern::Memory(self.handle.internal_handle())
    }
}

impl std::cmp::PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl std::cmp::Eq for Memory {}

impl<'a> Exportable<'a> for Memory {
    fn get_self_from_extern(_extern: &'a Extern) -> Result<&'a Self, ExportError> {
        match _extern {
            Extern::Memory(memory) => Ok(memory),
            _ => Err(ExportError::IncompatibleType),
        }
    }
}

/// Underlying buffer for a memory.
#[derive(Copy, Clone)]
pub(crate) struct MemoryBuffer<'a> {
    base: *mut u8,
    len: usize,
    marker: PhantomData<(&'a Memory, &'a ContextObjects)>,
}

impl<'a> MemoryBuffer<'a> {
    pub(crate) fn read(&self, offset: u64, buf: &mut [u8]) -> Result<(), MemoryAccessError> {
        let end = offset
            .checked_add(buf.len() as u64)
            .ok_or(MemoryAccessError::Overflow)?;
        if end > self.len.try_into().unwrap() {
            return Err(MemoryAccessError::HeapOutOfBounds);
        }
        unsafe {
            volatile_memcpy_read(self.base.add(offset as usize), buf.as_mut_ptr(), buf.len());
        }
        Ok(())
    }

    pub(crate) fn read_uninit<'b>(
        &self,
        offset: u64,
        buf: &'b mut [MaybeUninit<u8>],
    ) -> Result<&'b mut [u8], MemoryAccessError> {
        let end = offset
            .checked_add(buf.len() as u64)
            .ok_or(MemoryAccessError::Overflow)?;
        if end > self.len.try_into().unwrap() {
            return Err(MemoryAccessError::HeapOutOfBounds);
        }
        let buf_ptr = buf.as_mut_ptr() as *mut u8;
        unsafe {
            volatile_memcpy_read(self.base.add(offset as usize), buf_ptr, buf.len());
        }

        Ok(unsafe { slice::from_raw_parts_mut(buf_ptr, buf.len()) })
    }

    pub(crate) fn write(&self, offset: u64, data: &[u8]) -> Result<(), MemoryAccessError> {
        let end = offset
            .checked_add(data.len() as u64)
            .ok_or(MemoryAccessError::Overflow)?;
        if end > self.len.try_into().unwrap() {
            return Err(MemoryAccessError::HeapOutOfBounds);
        }
        unsafe {
            volatile_memcpy_write(data.as_ptr(), self.base.add(offset as usize), data.len());
        }
        Ok(())
    }
}

// We can't use a normal memcpy here because it has undefined behavior if the
// memory is being concurrently modified. So we need to write our own memcpy
// implementation which uses volatile operations.
//
// The implementation of these functions can optimize very well when inlined
// with a fixed length: they should compile down to a single load/store
// instruction for small (8/16/32/64-bit) copies.
#[inline]
unsafe fn volatile_memcpy_read(mut src: *const u8, mut dst: *mut u8, mut len: usize) {
    #[inline]
    unsafe fn copy_one<T>(src: &mut *const u8, dst: &mut *mut u8, len: &mut usize) {
        #[repr(packed)]
        struct Unaligned<T>(T);
        let val = (*src as *const Unaligned<T>).read_volatile();
        (*dst as *mut Unaligned<T>).write(val);
        *src = src.add(mem::size_of::<T>());
        *dst = dst.add(mem::size_of::<T>());
        *len -= mem::size_of::<T>();
    }

    while len >= 8 {
        copy_one::<u64>(&mut src, &mut dst, &mut len);
    }
    if len >= 4 {
        copy_one::<u32>(&mut src, &mut dst, &mut len);
    }
    if len >= 2 {
        copy_one::<u16>(&mut src, &mut dst, &mut len);
    }
    if len >= 1 {
        copy_one::<u8>(&mut src, &mut dst, &mut len);
    }
}
#[inline]
unsafe fn volatile_memcpy_write(mut src: *const u8, mut dst: *mut u8, mut len: usize) {
    #[inline]
    unsafe fn copy_one<T>(src: &mut *const u8, dst: &mut *mut u8, len: &mut usize) {
        #[repr(packed)]
        struct Unaligned<T>(T);
        let val = (*src as *const Unaligned<T>).read();
        (*dst as *mut Unaligned<T>).write_volatile(val);
        *src = src.add(mem::size_of::<T>());
        *dst = dst.add(mem::size_of::<T>());
        *len -= mem::size_of::<T>();
    }

    while len >= 8 {
        copy_one::<u64>(&mut src, &mut dst, &mut len);
    }
    if len >= 4 {
        copy_one::<u32>(&mut src, &mut dst, &mut len);
    }
    if len >= 2 {
        copy_one::<u16>(&mut src, &mut dst, &mut len);
    }
    if len >= 1 {
        copy_one::<u8>(&mut src, &mut dst, &mut len);
    }
}
