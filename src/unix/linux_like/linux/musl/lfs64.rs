#[inline(always)]
pub unsafe extern "C" fn creat64(path: *const ::c_char, mode: ::mode_t) -> ::c_int {
    ::creat(path, mode)
}

#[inline(always)]
pub unsafe extern "C" fn fallocate64(
    fd: ::c_int,
    mode: ::c_int,
    offset: ::off64_t,
    len: ::off64_t,
) -> ::c_int {
    ::fallocate(fd, mode, offset, len)
}

#[inline(always)]
pub unsafe extern "C" fn fgetpos64(stream: *mut ::FILE, pos: *mut ::fpos64_t) -> ::c_int {
    ::fgetpos(stream, pos.cast())
}

#[inline(always)]
pub unsafe extern "C" fn fopen64(pathname: *const ::c_char, mode: *const ::c_char) -> *mut ::FILE {
    ::fopen(pathname, mode)
}

#[inline(always)]
pub unsafe extern "C" fn freopen64(
    pathname: *const ::c_char,
    mode: *const ::c_char,
    stream: *mut ::FILE,
) -> *mut ::FILE {
    ::freopen(pathname, mode, stream)
}

#[inline(always)]
pub unsafe extern "C" fn fseeko64(
    stream: *mut ::FILE,
    offset: ::off64_t,
    whence: ::c_int,
) -> ::c_int {
    ::fseeko(stream, offset, whence)
}

#[inline(always)]
pub unsafe extern "C" fn fsetpos64(stream: *mut ::FILE, pos: *const ::fpos64_t) -> ::c_int {
    ::fsetpos(stream, pos.cast())
}

#[inline(always)]
pub unsafe extern "C" fn fstat64(fildes: ::c_int, buf: *mut ::stat64) -> ::c_int {
    ::fstat(fildes, buf.cast())
}

#[inline(always)]
pub unsafe extern "C" fn fstatat64(
    fd: ::c_int,
    path: *const ::c_char,
    buf: *mut ::stat64,
    flag: ::c_int,
) -> ::c_int {
    ::fstatat(fd, path, buf.cast(), flag)
}

#[inline(always)]
pub unsafe extern "C" fn fstatfs64(fd: ::c_int, buf: *mut ::statfs64) -> ::c_int {
    ::fstatfs(fd, buf.cast())
}

#[inline(always)]
pub unsafe extern "C" fn fstatvfs64(fd: ::c_int, buf: *mut ::statvfs64) -> ::c_int {
    ::fstatvfs(fd, buf.cast())
}

#[inline(always)]
pub unsafe extern "C" fn ftello64(stream: *mut ::FILE) -> ::off64_t {
    ::ftello(stream)
}

#[inline(always)]
pub unsafe extern "C" fn ftruncate64(fd: ::c_int, length: ::off64_t) -> ::c_int {
    ::ftruncate(fd, length)
}

#[inline(always)]
pub unsafe extern "C" fn getrlimit64(resource: ::c_int, rlim: *mut ::rlimit64) -> ::c_int {
    ::getrlimit(resource, rlim.cast())
}

#[inline(always)]
pub unsafe extern "C" fn lseek64(fd: ::c_int, offset: ::off64_t, whence: ::c_int) -> ::off64_t {
    ::lseek(fd, offset, whence)
}

#[inline(always)]
pub unsafe extern "C" fn lstat64(path: *const ::c_char, buf: *mut ::stat64) -> ::c_int {
    ::lstat(path, buf.cast())
}

#[inline(always)]
pub unsafe extern "C" fn mmap64(
    addr: *mut ::c_void,
    length: ::size_t,
    prot: ::c_int,
    flags: ::c_int,
    fd: ::c_int,
    offset: ::off64_t,
) -> *mut ::c_void {
    ::mmap(addr, length, prot, flags, fd, offset)
}

#[inline(always)]
pub unsafe extern "C" fn open64(
    pathname: *const ::c_char,
    flags: ::c_int,
    mode: ::mode_t,
) -> ::c_int {
    ::open(pathname, flags | ::O_LARGEFILE, mode)
}

#[inline(always)]
pub unsafe extern "C" fn openat64(
    dirfd: ::c_int,
    pathname: *const ::c_char,
    flags: ::c_int,
    mode: ::mode_t,
) -> ::c_int {
    ::openat(dirfd, pathname, flags | ::O_LARGEFILE, mode)
}

#[inline(always)]
pub unsafe extern "C" fn posix_fadvise64(
    fd: ::c_int,
    offset: ::off64_t,
    len: ::off64_t,
    advice: ::c_int,
) -> ::c_int {
    ::posix_fadvise(fd, offset, len, advice)
}

#[inline(always)]
pub unsafe extern "C" fn posix_fallocate64(
    fd: ::c_int,
    offset: ::off64_t,
    len: ::off64_t,
) -> ::c_int {
    ::posix_fallocate(fd, offset, len)
}

#[inline(always)]
pub unsafe extern "C" fn pread64(
    fd: ::c_int,
    buf: *mut ::c_void,
    count: ::size_t,
    offset: ::off64_t,
) -> ::ssize_t {
    ::pread(fd, buf, count, offset)
}

#[inline(always)]
pub unsafe extern "C" fn preadv64(
    fd: ::c_int,
    iov: *const ::iovec,
    iovcnt: ::c_int,
    offset: ::off64_t,
) -> ::ssize_t {
    ::preadv(fd, iov, iovcnt, offset)
}

#[inline(always)]
pub unsafe extern "C" fn prlimit64(
    pid: ::pid_t,
    resource: ::c_int,
    new_limit: *const ::rlimit64,
    old_limit: *mut ::rlimit64,
) -> ::c_int {
    ::prlimit(pid, resource, new_limit.cast(), old_limit.cast())
}

#[inline(always)]
pub unsafe extern "C" fn pwrite64(
    fd: ::c_int,
    buf: *const ::c_void,
    count: ::size_t,
    offset: ::off64_t,
) -> ::ssize_t {
    ::pwrite(fd, buf, count, offset)
}

#[inline(always)]
pub unsafe extern "C" fn pwritev64(
    fd: ::c_int,
    iov: *const ::iovec,
    iovcnt: ::c_int,
    offset: ::off64_t,
) -> ::ssize_t {
    ::pwritev(fd, iov, iovcnt, offset)
}

#[inline(always)]
pub unsafe extern "C" fn readdir64(dirp: *mut ::DIR) -> *mut ::dirent64 {
    ::readdir(dirp).cast()
}

#[inline(always)]
pub unsafe extern "C" fn readdir64_r(
    dirp: *mut ::DIR,
    entry: *mut ::dirent64,
    result: *mut *mut ::dirent64,
) -> ::c_int {
    ::readdir_r(dirp, entry.cast(), result.cast())
}

#[inline(always)]
pub unsafe extern "C" fn sendfile64(
    out_fd: ::c_int,
    in_fd: ::c_int,
    offset: *mut ::off64_t,
    count: ::size_t,
) -> ::ssize_t {
    ::sendfile(out_fd, in_fd, offset, count)
}

#[inline(always)]
pub unsafe extern "C" fn setrlimit64(resource: ::c_int, rlim: *const ::rlimit64) -> ::c_int {
    ::setrlimit(resource, rlim.cast())
}

#[inline(always)]
pub unsafe extern "C" fn stat64(pathname: *const ::c_char, statbuf: *mut ::stat64) -> ::c_int {
    ::stat(pathname, statbuf.cast())
}

#[inline(always)]
pub unsafe extern "C" fn statfs64(pathname: *const ::c_char, buf: *mut ::statfs64) -> ::c_int {
    ::statfs(pathname, buf.cast())
}

#[inline(always)]
pub unsafe extern "C" fn statvfs64(path: *const ::c_char, buf: *mut ::statvfs64) -> ::c_int {
    ::statvfs(path, buf.cast())
}

#[inline(always)]
pub unsafe extern "C" fn tmpfile64() -> *mut ::FILE {
    ::tmpfile()
}

#[inline(always)]
pub unsafe extern "C" fn truncate64(path: *const ::c_char, length: ::off64_t) -> ::c_int {
    ::truncate(path, length)
}
