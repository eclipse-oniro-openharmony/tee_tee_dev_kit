pub struct RwLock {}
 
pub type MovableRwLock = RwLock;
 
unsafe impl Send for RwLock {}
unsafe impl Sync for RwLock {}
 
impl RwLock {
    #[inline]
    pub const fn new() -> RwLock {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn read(&self) {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn write(&self) {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn read_unlock(&self) {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn write_unlock(&self) {
        panic!("Rwlock is unsupported.");
    }
}