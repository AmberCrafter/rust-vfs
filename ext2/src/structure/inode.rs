use bitflags::bitflags;

#[repr(C)]
pub struct Inode {
    mode: u16,
    uid: u16,
    size: u32,
    atime: u32,
    ctime: u32,
    mtime: u32,
    dtime: u32,
    gid: u16,
    links_count: u16,
    blocks: u32,
    flags: u32,
    osd1: u32,
    block: [[u8; 4]; 15],
    generation: u32,
    file_acl: u32,
    dir_acl: u32,
    faddr: u32,
    osd2: [u8; 12],
}

pub enum ReservedInode {
    Ext2BadIno = 1,
    Ext2RootIno = 2,
    Ext2AclIdxIno = 3,
    Ext2AclDataIco = 4,
    Ext2BootLoaderIno = 5,
    Ext2UndelDirIno = 6,
}

bitflags! {
    pub struct Mode: u16 {
        // file format
        const EXT2_S_IFSOCK = 0xC000;
        const EXT2_S_IFLNK = 0xA000;
        const EXT2_S_IFREG = 0x8000;
        const EXT2_S_IFBLK = 0x6000;
        const EXT2_S_IFDIR = 0x4000;
        const EXT2_S_IFCHR = 0x2000;
        const EXT2_S_IFIFO = 0x1000;
        // process execution user/group override
        const EXT2_S_ISUID = 0x0800;
        const EXT2_S_ISGID = 0x0400;
        const EXT2_S_ISVTX = 0x0200;
        // access rights
        const EXT2_S_IRUSR = 0x0100;
        const EXT2_S_IWUSR = 0x0080;
        const EXT2_S_IXUSR = 0x0040;
        const EXT2_S_IRGRP = 0x0020;
        const EXT2_S_IWGRP = 0x0010;
        const EXT2_S_IXGRP = 0x0008;
        const EXT2_S_IROTH = 0x0004;
        const EXT2_S_IWOTH = 0x0002;
        const EXT2_S_IXOTH = 0x0001;
    }
}

bitflags! {
    pub struct Flags: u32 {
        const EXT2_SECRM_FL = 0x0000_0001;
        const EXT2_UNRM_FL = 0x0000_0002;
        const EXT2_COMPR_FL = 0x0000_0004;
        const EXT2_SYNC_FL = 0x0000_0008;
        const EXT2_IMMUTABLE_FL = 0x0000_0010;
        const EXT2_APPEND_FL = 0x0000_0020;
        const EXT2_NODUMP_FL = 0x0000_0040;
        const EXT2_NOATIME_FL = 0x0000_0080;
        // reserved for compression usage
        const EXT2_DIRTY_FL = 0x0000_0100;
        const EXT2_COMPRBLK_FL = 0x0000_0200;
        const EXT2_NOCOMPR_FL = 0x0000_0400;
        const EXT2_ECOMPR_FL = 0x0000_0800;
        // end of compression flags
        const EXT2_BTREE_FL = 0x0000_1000;
        const EXT2_INDEX_FL = 0x0000_1000;
        const EXT2_IMAGIC_FL = 0x0000_2000;
        const EXT3_JOURNAL_DATA_FL = 0x0000_4000;
        const EXT2_RESERVED_FL = 0x8000_0000;
    }
}

#[repr(C)]
pub struct Osd2Hurd {
    frag: u8,
    fsize: u8,
    mode_high: u16,
    uid_high: u16,
    gid_high: u16,
    author: u32,
}

#[repr(C)]
pub struct Osd2Linux {
    frag: u8,
    fsize: u8,
    reserved1: [u8;2],
    uid_high: u16,
    gid_high: u16,
    reserved2: [u8;4],
}

#[repr(C)]
pub struct Osd2Masix {
    frag: u8,
    fsize: u8,
    reserved: [u8;10],
}