/*
// A FileMode represents the kind of tree entries used by git. It
// resembles regular file systems modes, although FileModes are
// considerably simpler (there are not so many), and there are some,
// like Submodule that has no file system equivalent.
type FileMode uint32

const (
    // Empty is used as the FileMode of tree elements when comparing
    // trees in the following situations:
    //
    // - the mode of tree elements before their creation.  - the mode of
    // tree elements after their deletion.  - the mode of unmerged
    // elements when checking the index.
    //
    // Empty has no file system equivalent.  As Empty is the zero value
    // of FileMode, it is also returned by New and
    // NewFromOsNewFromOSFileMode along with an error, when they fail.
    Empty FileMode = 0
    // Dir represent a Directory.
    Dir FileMode = 0040000
    // Regular represent non-executable files.  Please note this is not
    // the same as golang regular files, which include executable files.
    Regular FileMode = 0100644
    // Deprecated represent non-executable files with the group writable
    // bit set.  This mode was supported by the first versions of git,
    // but it has been deprecated nowadays.  This library uses them
    // internally, so you can read old packfiles, but will treat them as
    // Regulars when interfacing with the outside world.  This is the
    // standard git behaviour.
    Deprecated FileMode = 0100664
    // Executable represents executable files.
    Executable FileMode = 0100755
    // Symlink represents symbolic links to files.
    Symlink FileMode = 0120000
    // Submodule represents git submodules.  This mode has no file system
    // equivalent.
    Submodule FileMode = 0160000
)
*/
pub const EMPTY: u32 = 0;
pub const DIR: u32 = 0040000;
pub const REGULAR: u32 = 100644;
pub const DEPRECATED: u32 = 100664;
pub const EXECUTABLE: u32 = 100755;
pub const SYMLINK: u32 = 120000;
pub const SUBMODULE: u32 = 160000;
