#![deny(dead_code)]
use crate::{WasiEnv, WasiError, WasiState, WasiThread};
use wasmer::{ContextMut, Memory, Memory32, MemorySize, WasmPtr, WasmSlice};
use wasmer_wasi_types::*;

type MemoryType = Memory32;
type MemoryOffset = u32;

pub(crate) fn args_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    argv: WasmPtr<WasmPtr<u8, MemoryType>, MemoryType>,
    argv_buf: WasmPtr<u8, MemoryType>,
) -> __wasi_errno_t {
    super::args_get::<MemoryType, T>(ctx, argv, argv_buf)
}

pub(crate) fn args_sizes_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    argc: WasmPtr<MemoryOffset, MemoryType>,
    argv_buf_size: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::args_sizes_get::<MemoryType, T>(ctx, argc, argv_buf_size)
}

pub(crate) fn clock_res_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    clock_id: __wasi_clockid_t,
    resolution: WasmPtr<__wasi_timestamp_t, MemoryType>,
) -> __wasi_errno_t {
    super::clock_res_get::<MemoryType, T>(ctx, clock_id, resolution)
}

pub(crate) fn clock_time_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    clock_id: __wasi_clockid_t,
    precision: __wasi_timestamp_t,
    time: WasmPtr<__wasi_timestamp_t, MemoryType>,
) -> __wasi_errno_t {
    super::clock_time_get::<MemoryType, T>(ctx, clock_id, precision, time)
}

pub(crate) fn environ_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    environ: WasmPtr<WasmPtr<u8, MemoryType>, MemoryType>,
    environ_buf: WasmPtr<u8, MemoryType>,
) -> __wasi_errno_t {
    super::environ_get::<MemoryType, T>(ctx, environ, environ_buf)
}

pub(crate) fn environ_sizes_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    environ_count: WasmPtr<MemoryOffset, MemoryType>,
    environ_buf_size: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::environ_sizes_get::<MemoryType, T>(ctx, environ_count, environ_buf_size)
}

pub(crate) fn fd_advise<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    offset: __wasi_filesize_t,
    len: __wasi_filesize_t,
    advice: __wasi_advice_t,
) -> __wasi_errno_t {
    super::fd_advise(ctx, fd, offset, len, advice)
}

pub(crate) fn fd_allocate<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    offset: __wasi_filesize_t,
    len: __wasi_filesize_t,
) -> __wasi_errno_t {
    super::fd_allocate(ctx, fd, offset, len)
}

pub(crate) fn fd_close<T>(ctx: ContextMut<'_, WasiEnv, T>, fd: __wasi_fd_t) -> __wasi_errno_t {
    super::fd_close(ctx, fd)
}

pub(crate) fn fd_datasync<T>(ctx: ContextMut<'_, WasiEnv, T>, fd: __wasi_fd_t) -> __wasi_errno_t {
    super::fd_datasync(ctx, fd)
}

pub(crate) fn fd_fdstat_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    buf_ptr: WasmPtr<__wasi_fdstat_t, MemoryType>,
) -> __wasi_errno_t {
    super::fd_fdstat_get::<MemoryType, T>(ctx, fd, buf_ptr)
}

pub(crate) fn fd_fdstat_set_flags<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    flags: __wasi_fdflags_t,
) -> __wasi_errno_t {
    super::fd_fdstat_set_flags(ctx, fd, flags)
}

pub(crate) fn fd_fdstat_set_rights<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    fs_rights_base: __wasi_rights_t,
    fs_rights_inheriting: __wasi_rights_t,
) -> __wasi_errno_t {
    super::fd_fdstat_set_rights(ctx, fd, fs_rights_base, fs_rights_inheriting)
}

pub(crate) fn fd_filestat_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    buf: WasmPtr<__wasi_filestat_t, MemoryType>,
) -> __wasi_errno_t {
    super::fd_filestat_get::<MemoryType, T>(ctx, fd, buf)
}

pub(crate) fn fd_filestat_set_size<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    st_size: __wasi_filesize_t,
) -> __wasi_errno_t {
    super::fd_filestat_set_size(ctx, fd, st_size)
}

pub(crate) fn fd_filestat_set_times<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    st_atim: __wasi_timestamp_t,
    st_mtim: __wasi_timestamp_t,
    fst_flags: __wasi_fstflags_t,
) -> __wasi_errno_t {
    super::fd_filestat_set_times(ctx, fd, st_atim, st_mtim, fst_flags)
}

pub(crate) fn fd_pread<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    iovs: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    offset: __wasi_filesize_t,
    nread: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::fd_pread::<MemoryType, T>(ctx, fd, iovs, iovs_len, offset, nread)
}

pub(crate) fn fd_prestat_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    buf: WasmPtr<__wasi_prestat_t, MemoryType>,
) -> __wasi_errno_t {
    super::fd_prestat_get::<MemoryType, T>(ctx, fd, buf)
}

pub(crate) fn fd_prestat_dir_name<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::fd_prestat_dir_name::<MemoryType, T>(ctx, fd, path, path_len)
}

pub(crate) fn fd_pwrite<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    iovs: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    offset: __wasi_filesize_t,
    nwritten: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::fd_pwrite::<MemoryType, T>(ctx, fd, iovs, iovs_len, offset, nwritten)
}

pub(crate) fn fd_read<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    iovs: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    nread: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::fd_read::<MemoryType, T>(ctx, fd, iovs, iovs_len, nread)
}

pub(crate) fn fd_readdir<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    cookie: __wasi_dircookie_t,
    bufused: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::fd_readdir::<MemoryType, T>(ctx, fd, buf, buf_len, cookie, bufused)
}

pub(crate) fn fd_renumber<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    from: __wasi_fd_t,
    to: __wasi_fd_t,
) -> __wasi_errno_t {
    super::fd_renumber(ctx, from, to)
}

pub(crate) fn fd_seek<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    offset: __wasi_filedelta_t,
    whence: __wasi_whence_t,
    newoffset: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::fd_seek::<MemoryType, T>(ctx, fd, offset, whence, newoffset)
}

pub(crate) fn fd_sync<T>(ctx: ContextMut<'_, WasiEnv, T>, fd: __wasi_fd_t) -> __wasi_errno_t {
    super::fd_sync(ctx, fd)
}

pub(crate) fn fd_tell<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    offset: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> __wasi_errno_t {
    super::fd_tell::<MemoryType, T>(ctx, fd, offset)
}

pub(crate) fn fd_write<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    iovs: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    nwritten: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::fd_write::<MemoryType, T>(ctx, fd, iovs, iovs_len, nwritten)
}

pub(crate) fn path_create_directory<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::path_create_directory::<MemoryType, T>(ctx, fd, path, path_len)
}

pub(crate) fn path_filestat_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    flags: __wasi_lookupflags_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    buf: WasmPtr<__wasi_filestat_t, MemoryType>,
) -> __wasi_errno_t {
    super::path_filestat_get::<MemoryType, T>(ctx, fd, flags, path, path_len, buf)
}

pub(crate) fn path_filestat_set_times<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    flags: __wasi_lookupflags_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    st_atim: __wasi_timestamp_t,
    st_mtim: __wasi_timestamp_t,
    fst_flags: __wasi_fstflags_t,
) -> __wasi_errno_t {
    super::path_filestat_set_times::<MemoryType, T>(
        ctx, fd, flags, path, path_len, st_atim, st_mtim, fst_flags,
    )
}

pub(crate) fn path_link<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    old_fd: __wasi_fd_t,
    old_flags: __wasi_lookupflags_t,
    old_path: WasmPtr<u8, MemoryType>,
    old_path_len: MemoryOffset,
    new_fd: __wasi_fd_t,
    new_path: WasmPtr<u8, MemoryType>,
    new_path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::path_link::<MemoryType, T>(
        ctx,
        old_fd,
        old_flags,
        old_path,
        old_path_len,
        new_fd,
        new_path,
        new_path_len,
    )
}

pub(crate) fn path_open<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    dirfd: __wasi_fd_t,
    dirflags: __wasi_lookupflags_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    o_flags: __wasi_oflags_t,
    fs_rights_base: __wasi_rights_t,
    fs_rights_inheriting: __wasi_rights_t,
    fs_flags: __wasi_fdflags_t,
    fd: WasmPtr<__wasi_fd_t, MemoryType>,
) -> __wasi_errno_t {
    super::path_open::<MemoryType, T>(
        ctx,
        dirfd,
        dirflags,
        path,
        path_len,
        o_flags,
        fs_rights_base,
        fs_rights_inheriting,
        fs_flags,
        fd,
    )
}

pub(crate) fn path_readlink<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    dir_fd: __wasi_fd_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    buf_used: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::path_readlink::<MemoryType, T>(ctx, dir_fd, path, path_len, buf, buf_len, buf_used)
}

pub(crate) fn path_remove_directory<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::path_remove_directory::<MemoryType, T>(ctx, fd, path, path_len)
}

pub(crate) fn path_rename<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    old_fd: __wasi_fd_t,
    old_path: WasmPtr<u8, MemoryType>,
    old_path_len: MemoryOffset,
    new_fd: __wasi_fd_t,
    new_path: WasmPtr<u8, MemoryType>,
    new_path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::path_rename::<MemoryType, T>(
        ctx,
        old_fd,
        old_path,
        old_path_len,
        new_fd,
        new_path,
        new_path_len,
    )
}

pub(crate) fn path_symlink<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    old_path: WasmPtr<u8, MemoryType>,
    old_path_len: MemoryOffset,
    fd: __wasi_fd_t,
    new_path: WasmPtr<u8, MemoryType>,
    new_path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::path_symlink::<MemoryType, T>(ctx, old_path, old_path_len, fd, new_path, new_path_len)
}

pub(crate) fn path_unlink_file<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::path_unlink_file::<MemoryType, T>(ctx, fd, path, path_len)
}

pub(crate) fn poll_oneoff<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    in_: WasmPtr<__wasi_subscription_t, MemoryType>,
    out_: WasmPtr<__wasi_event_t, MemoryType>,
    nsubscriptions: MemoryOffset,
    nevents: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::poll_oneoff::<MemoryType, T>(ctx, in_, out_, nsubscriptions, nevents)
}

pub(crate) fn proc_exit<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    code: __wasi_exitcode_t,
) -> Result<(), WasiError> {
    super::proc_exit(ctx, code)
}

pub(crate) fn proc_raise<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sig: __wasi_signal_t,
) -> __wasi_errno_t {
    super::proc_raise(ctx, sig)
}

pub(crate) fn random_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
) -> __wasi_errno_t {
    super::random_get::<MemoryType, T>(ctx, buf, buf_len)
}

pub(crate) fn fd_dup<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    fd: __wasi_fd_t,
    ret_fd: WasmPtr<__wasi_fd_t, MemoryType>,
) -> __wasi_errno_t {
    super::fd_dup::<MemoryType, T>(ctx, fd, ret_fd)
}

pub(crate) fn fd_event<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    initial_val: u64,
    flags: __wasi_eventfdflags,
    ret_fd: WasmPtr<__wasi_fd_t, MemoryType>,
) -> __wasi_errno_t {
    super::fd_event(ctx, initial_val, flags, ret_fd)
}

pub(crate) fn fd_pipe<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    ro_fd1: WasmPtr<__wasi_fd_t, MemoryType>,
    ro_fd2: WasmPtr<__wasi_fd_t, MemoryType>,
) -> __wasi_errno_t {
    super::fd_pipe::<MemoryType, T>(ctx, ro_fd1, ro_fd2)
}

pub(crate) fn tty_get<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    tty_state: WasmPtr<__wasi_tty_t, MemoryType>,
) -> __wasi_errno_t {
    super::tty_get::<MemoryType, T>(ctx, tty_state)
}

pub(crate) fn tty_set<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    tty_state: WasmPtr<__wasi_tty_t, MemoryType>,
) -> __wasi_errno_t {
    super::tty_set::<MemoryType, T>(ctx, tty_state)
}

pub(crate) fn getcwd<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    path: WasmPtr<u8, MemoryType>,
    path_len: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::getcwd::<MemoryType, T>(ctx, path, path_len)
}

pub(crate) fn chdir<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> __wasi_errno_t {
    super::chdir::<MemoryType, T>(ctx, path, path_len)
}

pub(crate) fn thread_spawn<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    method: WasmPtr<u8, MemoryType>,
    method_len: MemoryOffset,
    user_data: u64,
    reactor: __wasi_bool_t,
    ret_tid: WasmPtr<__wasi_tid_t, MemoryType>,
) -> __wasi_errno_t {
    super::thread_spawn::<MemoryType, T>(ctx, method, method_len, user_data, reactor, ret_tid)
}

pub(crate) fn thread_sleep<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    duration: __wasi_timestamp_t,
) -> Result<__wasi_errno_t, WasiError> {
    super::thread_sleep(ctx, duration)
}

pub(crate) fn thread_id<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    ret_tid: WasmPtr<__wasi_tid_t, MemoryType>,
) -> __wasi_errno_t {
    super::thread_id::<MemoryType, T>(ctx, ret_tid)
}

pub(crate) fn thread_join<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    tid: __wasi_tid_t,
) -> Result<__wasi_errno_t, WasiError> {
    super::thread_join(ctx, tid)
}

pub(crate) fn thread_parallelism<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    ret_parallelism: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::thread_parallelism::<MemoryType, T>(ctx, ret_parallelism)
}

pub(crate) fn thread_exit<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    exitcode: __wasi_exitcode_t,
) -> Result<__wasi_errno_t, WasiError> {
    super::thread_exit(ctx, exitcode)
}

pub(crate) fn sched_yield<T>(ctx: ContextMut<'_, WasiEnv, T>) -> Result<__wasi_errno_t, WasiError> {
    super::sched_yield(ctx)
}

pub(crate) fn getpid<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    ret_pid: WasmPtr<__wasi_pid_t, MemoryType>,
) -> __wasi_errno_t {
    super::getpid::<MemoryType, T>(ctx, ret_pid)
}

pub(crate) fn process_spawn<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    name: WasmPtr<u8, MemoryType>,
    name_len: MemoryOffset,
    chroot: __wasi_bool_t,
    args: WasmPtr<u8, MemoryType>,
    args_len: MemoryOffset,
    preopen: WasmPtr<u8, MemoryType>,
    preopen_len: MemoryOffset,
    stdin: __wasi_stdiomode_t,
    stdout: __wasi_stdiomode_t,
    stderr: __wasi_stdiomode_t,
    working_dir: WasmPtr<u8, MemoryType>,
    working_dir_len: MemoryOffset,
    ret_handles: WasmPtr<__wasi_bus_handles_t, MemoryType>,
) -> __bus_errno_t {
    super::process_spawn::<MemoryType, T>(
        ctx,
        name,
        name_len,
        chroot,
        args,
        args_len,
        preopen,
        preopen_len,
        stdin,
        stdout,
        stderr,
        working_dir,
        working_dir_len,
        ret_handles,
    )
}

pub(crate) fn bus_open_local<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    name: WasmPtr<u8, MemoryType>,
    name_len: MemoryOffset,
    reuse: __wasi_bool_t,
    ret_bid: WasmPtr<__wasi_bid_t, MemoryType>,
) -> __bus_errno_t {
    super::bus_open_local::<MemoryType, T>(ctx, name, name_len, reuse, ret_bid)
}

pub(crate) fn bus_open_remote<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    name: WasmPtr<u8, MemoryType>,
    name_len: MemoryOffset,
    reuse: __wasi_bool_t,
    instance: WasmPtr<u8, MemoryType>,
    instance_len: MemoryOffset,
    token: WasmPtr<u8, MemoryType>,
    token_len: MemoryOffset,
    ret_bid: WasmPtr<__wasi_bid_t, MemoryType>,
) -> __bus_errno_t {
    super::bus_open_remote::<MemoryType, T>(
        ctx,
        name,
        name_len,
        reuse,
        instance,
        instance_len,
        token,
        token_len,
        ret_bid,
    )
}

pub(crate) fn bus_close<T>(ctx: ContextMut<'_, WasiEnv, T>, bid: __wasi_bid_t) -> __bus_errno_t {
    super::bus_close(ctx, bid)
}

pub(crate) fn bus_call<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    bid: __wasi_bid_t,
    keep_alive: __wasi_bool_t,
    topic: WasmPtr<u8, MemoryType>,
    topic_len: MemoryOffset,
    format: __wasi_busdataformat_t,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    ret_cid: WasmPtr<__wasi_cid_t, MemoryType>,
) -> __bus_errno_t {
    super::bus_call::<MemoryType, T>(
        ctx, bid, keep_alive, topic, topic_len, format, buf, buf_len, ret_cid,
    )
}

pub(crate) fn bus_subcall<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    parent: __wasi_cid_t,
    keep_alive: __wasi_bool_t,
    topic: WasmPtr<u8, MemoryType>,
    topic_len: MemoryOffset,
    format: __wasi_busdataformat_t,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    ret_cid: WasmPtr<__wasi_cid_t, MemoryType>,
) -> __bus_errno_t {
    super::bus_subcall::<MemoryType, T>(
        ctx, parent, keep_alive, topic, topic_len, format, buf, buf_len, ret_cid,
    )
}

pub(crate) fn bus_poll<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    timeout: __wasi_timestamp_t,
    events: WasmPtr<u8, MemoryType>,
    nevents: MemoryOffset,
    malloc: WasmPtr<u8, MemoryType>,
    malloc_len: MemoryOffset,
    ret_nevents: WasmPtr<MemoryOffset, MemoryType>,
) -> __bus_errno_t {
    super::bus_poll::<MemoryType, T>(
        ctx,
        timeout,
        events,
        nevents,
        malloc,
        malloc_len,
        ret_nevents,
    )
}

pub(crate) fn call_reply<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    cid: __wasi_cid_t,
    format: __wasi_busdataformat_t,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
) -> __bus_errno_t {
    super::call_reply::<MemoryType, T>(ctx, cid, format, buf, buf_len)
}

pub(crate) fn call_fault<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    cid: __wasi_cid_t,
    fault: __bus_errno_t,
) -> __bus_errno_t {
    super::call_fault(ctx, cid, fault)
}

pub(crate) fn call_close<T>(ctx: ContextMut<'_, WasiEnv, T>, cid: __wasi_cid_t) -> __bus_errno_t {
    super::call_close(ctx, cid)
}

pub(crate) fn port_bridge<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    network: WasmPtr<u8, MemoryType>,
    network_len: MemoryOffset,
    token: WasmPtr<u8, MemoryType>,
    token_len: MemoryOffset,
    security: __wasi_streamsecurity_t,
) -> __wasi_errno_t {
    super::port_bridge::<MemoryType, T>(ctx, network, network_len, token, token_len, security)
}

pub(crate) fn port_unbridge<T>(ctx: ContextMut<'_, WasiEnv, T>) -> __wasi_errno_t {
    super::port_unbridge(ctx)
}

pub(crate) fn port_dhcp_acquire<T>(ctx: ContextMut<'_, WasiEnv, T>) -> __wasi_errno_t {
    super::port_dhcp_acquire(ctx)
}

pub(crate) fn port_addr_add<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    addr: WasmPtr<__wasi_cidr_t, MemoryType>,
) -> __wasi_errno_t {
    super::port_addr_add::<MemoryType, T>(ctx, addr)
}

pub(crate) fn port_addr_remove<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    addr: WasmPtr<__wasi_addr_t, MemoryType>,
) -> __wasi_errno_t {
    super::port_addr_remove::<MemoryType, T>(ctx, addr)
}

pub(crate) fn port_addr_clear<T>(ctx: ContextMut<'_, WasiEnv, T>) -> __wasi_errno_t {
    super::port_addr_clear(ctx)
}

pub(crate) fn port_addr_list<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    addrs: WasmPtr<__wasi_cidr_t, MemoryType>,
    naddrs: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::port_addr_list::<MemoryType, T>(ctx, addrs, naddrs)
}

pub(crate) fn port_mac<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    ret_mac: WasmPtr<__wasi_hardwareaddress_t, MemoryType>,
) -> __wasi_errno_t {
    super::port_mac::<MemoryType, T>(ctx, ret_mac)
}

pub(crate) fn port_gateway_set<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    ip: WasmPtr<__wasi_addr_t, MemoryType>,
) -> __wasi_errno_t {
    super::port_gateway_set::<MemoryType, T>(ctx, ip)
}

pub(crate) fn port_route_add<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    cidr: WasmPtr<__wasi_cidr_t, MemoryType>,
    via_router: WasmPtr<__wasi_addr_t, MemoryType>,
    preferred_until: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
    expires_at: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
) -> __wasi_errno_t {
    super::port_route_add::<MemoryType, T>(ctx, cidr, via_router, preferred_until, expires_at)
}

pub(crate) fn port_route_remove<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    ip: WasmPtr<__wasi_addr_t, MemoryType>,
) -> __wasi_errno_t {
    super::port_route_remove::<MemoryType, T>(ctx, ip)
}

pub(crate) fn port_route_clear<T>(ctx: ContextMut<'_, WasiEnv, T>) -> __wasi_errno_t {
    super::port_route_clear(ctx)
}

pub(crate) fn port_route_list<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    routes: WasmPtr<__wasi_route_t, MemoryType>,
    nroutes: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::port_route_list::<MemoryType, T>(ctx, routes, nroutes)
}

pub(crate) fn ws_connect<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    url: WasmPtr<u8, MemoryType>,
    url_len: MemoryOffset,
    ret_sock: WasmPtr<__wasi_fd_t, MemoryType>,
) -> __wasi_errno_t {
    super::ws_connect::<MemoryType, T>(ctx, url, url_len, ret_sock)
}

pub(crate) fn http_request<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    url: WasmPtr<u8, MemoryType>,
    url_len: MemoryOffset,
    method: WasmPtr<u8, MemoryType>,
    method_len: MemoryOffset,
    headers: WasmPtr<u8, MemoryType>,
    headers_len: MemoryOffset,
    gzip: __wasi_bool_t,
    ret_handles: WasmPtr<__wasi_http_handles_t, MemoryType>,
) -> __wasi_errno_t {
    super::http_request::<MemoryType, T>(
        ctx,
        url,
        url_len,
        method,
        method_len,
        headers,
        headers_len,
        gzip,
        ret_handles,
    )
}

pub(crate) fn http_status<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    status: WasmPtr<__wasi_http_status_t, MemoryType>,
    status_text: WasmPtr<u8, MemoryType>,
    status_text_len: WasmPtr<MemoryOffset, MemoryType>,
    headers: WasmPtr<u8, MemoryType>,
    headers_len: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::http_status::<MemoryType, T>(ctx, sock, status)
}

pub(crate) fn sock_status<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    ret_status: WasmPtr<__wasi_sockstatus_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_status::<MemoryType, T>(ctx, sock, ret_status)
}

pub(crate) fn sock_addr_local<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    ret_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_addr_local::<MemoryType, T>(ctx, sock, ret_addr)
}

pub(crate) fn sock_addr_peer<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    ro_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_addr_peer::<MemoryType, T>(ctx, sock, ro_addr)
}

pub(crate) fn sock_open<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    af: __wasi_addressfamily_t,
    ty: __wasi_socktype_t,
    pt: __wasi_sockproto_t,
    ro_sock: WasmPtr<__wasi_fd_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_open::<MemoryType, T>(ctx, af, ty, pt, ro_sock)
}

pub(crate) fn sock_set_opt_flag<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    opt: __wasi_sockoption_t,
    flag: __wasi_bool_t,
) -> __wasi_errno_t {
    super::sock_set_opt_flag(ctx, sock, opt, flag)
}

pub(crate) fn sock_get_opt_flag<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    opt: __wasi_sockoption_t,
    ret_flag: WasmPtr<__wasi_bool_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_get_opt_flag::<MemoryType, T>(ctx, sock, opt, ret_flag)
}

pub fn sock_set_opt_time<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    opt: __wasi_sockoption_t,
    time: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_set_opt_time(ctx, sock, opt, time)
}

pub fn sock_get_opt_time<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    opt: __wasi_sockoption_t,
    ret_time: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_get_opt_time(ctx, sock, opt, ret_time)
}

pub fn sock_set_opt_size<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    opt: __wasi_sockoption_t,
    size: __wasi_filesize_t,
) -> __wasi_errno_t {
    super::sock_set_opt_size(ctx, sock, opt, size)
}

pub fn sock_get_opt_size<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    opt: __wasi_sockoption_t,
    ret_size: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_get_opt_size(ctx, sock, opt, ret_size)
}

pub(crate) fn sock_join_multicast_v4<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    multiaddr: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
    iface: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_join_multicast_v4::<MemoryType, T>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_leave_multicast_v4<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    multiaddr: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
    iface: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_leave_multicast_v4::<MemoryType, T>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_join_multicast_v6<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    multiaddr: WasmPtr<__wasi_addr_ip6_t, MemoryType>,
    iface: u32,
) -> __wasi_errno_t {
    super::sock_join_multicast_v6::<MemoryType, T>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_leave_multicast_v6<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    multiaddr: WasmPtr<__wasi_addr_ip6_t, MemoryType>,
    iface: u32,
) -> __wasi_errno_t {
    super::sock_leave_multicast_v6::<MemoryType, T>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_bind<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_bind::<MemoryType, T>(ctx, sock, addr)
}

pub(crate) fn sock_listen<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    backlog: MemoryOffset,
) -> __wasi_errno_t {
    super::sock_listen::<MemoryType, T>(ctx, sock, backlog)
}

pub(crate) fn sock_accept<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    fd_flags: __wasi_fdflags_t,
    ro_fd: WasmPtr<__wasi_fd_t, MemoryType>,
    ro_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::sock_accept::<MemoryType, T>(ctx, sock, fd_flags, ro_fd, ro_addr)
}

pub(crate) fn sock_connect<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> __wasi_errno_t {
    super::sock_connect::<MemoryType, T>(ctx, sock, addr)
}

pub(crate) fn sock_recv<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    ri_data: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    ri_data_len: MemoryOffset,
    ri_flags: __wasi_riflags_t,
    ro_data_len: WasmPtr<MemoryOffset, MemoryType>,
    ro_flags: WasmPtr<__wasi_roflags_t, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::sock_recv::<MemoryType, T>(
        ctx,
        sock,
        ri_data,
        ri_data_len,
        ri_flags,
        ro_data_len,
        ro_flags,
    )
}

pub(crate) fn sock_recv_from<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    ri_data: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    ri_data_len: MemoryOffset,
    ri_flags: __wasi_riflags_t,
    ro_data_len: WasmPtr<MemoryOffset, MemoryType>,
    ro_flags: WasmPtr<__wasi_roflags_t, MemoryType>,
    ro_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::sock_recv_from::<MemoryType, T>(
        ctx,
        sock,
        ri_data,
        ri_data_len,
        ri_flags,
        ro_data_len,
        ro_flags,
        ro_addr,
    )
}

pub(crate) fn sock_send<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    si_data: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    si_data_len: MemoryOffset,
    si_flags: __wasi_siflags_t,
    ret_data_len: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::sock_send::<MemoryType, T>(ctx, sock, si_data, si_data_len, si_flags, ret_data_len)
}

pub(crate) fn sock_send_to<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    si_data: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    si_data_len: MemoryOffset,
    si_flags: __wasi_siflags_t,
    addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
    ret_data_len: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    super::sock_send_to::<MemoryType, T>(
        ctx,
        sock,
        si_data,
        si_data_len,
        si_flags,
        addr,
        ret_data_len,
    )
}

pub(crate) fn sock_send_file<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    out_fd: __wasi_fd_t,
    in_fd: __wasi_fd_t,
    offset: __wasi_filesize_t,
    count: __wasi_filesize_t,
    ret_sent: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> Result<__wasi_errno_t, WasiError> {
    unsafe { super::sock_send_file::<MemoryType, T>(ctx, out_fd, in_fd, offset, count, ret_sent) }
}

pub(crate) fn sock_shutdown<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    sock: __wasi_fd_t,
    how: __wasi_sdflags_t,
) -> __wasi_errno_t {
    super::sock_shutdown(ctx, sock, how)
}

pub(crate) fn resolve<T>(
    ctx: ContextMut<'_, WasiEnv, T>,
    host: WasmPtr<u8, MemoryType>,
    host_len: MemoryOffset,
    port: u16,
    ips: WasmPtr<__wasi_addr_t, MemoryType>,
    nips: MemoryOffset,
    ret_nips: WasmPtr<MemoryOffset, MemoryType>,
) -> __wasi_errno_t {
    super::resolve::<MemoryType, T>(ctx, host, host_len, port, ips, nips, ret_nips)
}
