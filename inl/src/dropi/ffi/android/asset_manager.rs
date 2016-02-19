use libc;

/* automatically generated by rust-bindgen */

pub enum AAssetManager { }
pub enum AAssetDir { }
pub enum AAsset { }
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum AAssetMode {
    UNKNOWN = 0,
    RANDOM = 1,
    STREAMING = 2,
    BUFFER = 3,
}
extern "C" {
    pub fn AAssetManager_openDir(mgr: *mut AAssetManager,
                                 dirName: *const ::std::os::raw::c_char)
     -> *mut AAssetDir;
    pub fn AAssetManager_open(mgr: *mut AAssetManager,
                              filename: *const ::std::os::raw::c_char,
                              mode: ::std::os::raw::c_int) -> *mut AAsset;
    pub fn AAssetDir_getNextFileName(assetDir: *mut AAssetDir)
     -> *const ::std::os::raw::c_char;
    pub fn AAssetDir_rewind(assetDir: *mut AAssetDir);
    pub fn AAssetDir_close(assetDir: *mut AAssetDir);
    pub fn AAsset_read(asset: *mut AAsset, buf: *mut ::std::os::raw::c_void,
                       count: usize) -> ::std::os::raw::c_int;
    pub fn AAsset_seek(asset: *mut AAsset, offset: libc::off_t,
                       whence: ::std::os::raw::c_int) -> libc::off_t;
    pub fn AAsset_seek64(asset: *mut AAsset, offset: u64,
                         whence: ::std::os::raw::c_int) -> u64;
    pub fn AAsset_close(asset: *mut AAsset);
    pub fn AAsset_getBuffer(asset: *mut AAsset)
     -> *const ::std::os::raw::c_void;
    pub fn AAsset_getLength(asset: *mut AAsset) -> libc::off_t;
    pub fn AAsset_getLength64(asset: *mut AAsset) -> u64;
    pub fn AAsset_getRemainingLength(asset: *mut AAsset) -> libc::off_t;
    pub fn AAsset_getRemainingLength64(asset: *mut AAsset) -> u64;
    pub fn AAsset_openFileDescriptor(asset: *mut AAsset, outStart: *mut libc::off_t,
                                     outLength: *mut libc::off_t)
     -> ::std::os::raw::c_int;
    pub fn AAsset_openFileDescriptor64(asset: *mut AAsset,
                                       outStart: *mut u64,
                                       outLength: *mut u64)
     -> ::std::os::raw::c_int;
    pub fn AAsset_isAllocated(asset: *mut AAsset) -> ::std::os::raw::c_int;
}
