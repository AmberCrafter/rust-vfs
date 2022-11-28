use bitflags::bitflags;

#[repr(C)]
struct SuperBlock {
    inodes_count: u32,
    blocks_count: u32,
    r_blocks_count: u32,
    free_block_count: u32,
    free_inodes_count: u32,
    first_data_block: u32,
    log_block_size: u32,
    log_frag_size: u32, // if (positive) {freg_size = 1024<<log_frag_size} else {freg_size = 1024>>-slog_frag_size}
    blocks_per_group: u32,
    frags_per_group: u32,
    inodes_per_group: u32,
    mtime: u32, // POSIX
    wtime: u32, // POSIX
    mnt_count: u16,
    max_mnt_count: u16,
    magic: u16,
    state: u16,
    errors: u16,
    minor_rev_level: u16,
    lastcheck: u32,
    checkinterval: u32,
    creator_os: u32,
    rev_level: u32,
    def_resuid: u16,
    def_resgid: u16,
    // EXT2_DYNAMIC_REV Specific
    first_ino: u32,
    inode_size: u16,
    block_group_nr: u16,
    feature_compat: u32,
    feature_incompat: u32,
    feature_ro_compat: u32,
    uuid: [u8; 16],
    volume_name: [u8; 16],
    last_mounted: [u8; 64],
    algo_bitmap: u32,
    // Performance Hints
    peralloc_blocks: u8,
    prealloc_dir_blocks: u8,
    alignment: [u8; 2],
    // Journaling Support
    journal_uuid: [u8; 16],
    journal_inum: u32,
    journal_dev: u32,
    last_orphan: u32,
    // Directory Indexing Support
    hash_seed: [[u8; 4]; 4],
    def_hash_version: u8,
    padding: [u8; 3],
    // Other options
    defualt_mount_options: u32,
    first_meta_bg: u32,
    unused: [u8; 760],
}

pub enum State {
    Ext2ValidFs = 1,
    Ext2ErrorFs = 2,
}

pub enum Errors {
    Ext2ErrorsContinue = 1,
    Ext2ErrorsRO = 2,
    Ext2ErrorsPanic = 3,
}

pub enum CreatorOS {
    Ext2OSLinux = 0,
    Ext2OSHurd = 1,
    Ext2OSMasix = 2,
    Ext2OSFreebsd = 3,
    Ext2OSLites = 4,
}

pub enum RevLevel {
    Ext2GoodOldRev = 0,
    Ext2DynamicRev = 1,
}

bitflags! {
    struct FeatureCompat: u32 {
        const EXT2_FEATURE_COMPAT_DIR_PREALLOC = 1<<0;
        const EXT2_FEATURE_COMPAT_IMAGIC_INODES = 1<<1;
        const EXT2_FEATURE_COMPAT_HAS_JOURNAL = 1<<2;
        const EXT2_FEATURE_COMPAT_EXT_ATTR = 1<<3;
        const EXT2_FEATURE_COMPAT_RESIZE_INO = 1<<3 | 1<<1;
        const EXT2_FEATURE_COMPAT_DIR_INDEX = 1<<4 | 1<<2;
    }
}

bitflags! {
    struct FeatureIncompat: u32 {
        const EXT2_FEATURE_INCOMPAT_COMPRESSION = 1<<0;
        const EXT2_FEATURE_INCOMPAT_FILETYPE = 1<<1;
        const EXT2_FEATURE_INCOMPAT_RECOVER = 1<<2;
        const EXT2_FEATURE_INCOMPAT_JOURNAL_DEV = 1<<3;
        const EXT2_FEATURE_INCOMPAT_META_BG = 1<<3 | 1<<1;
    }
}

bitflags! {
    struct FeatureROCompat: u32 {
        const EXT2_FEATURE_RO_COMPAT_SPARSE_SUPER = 1<<0;
        const EXT2_FEATURE_RO_COMPAT_LARGE_FILE = 1<<1;
        const EXT2_FEATURE_RO_COMPAT_BTREE_DIR = 1<<2;
    }
}

bitflags! {
    struct AlgoBitmap:u32 {
        const EXT2_LZV1_ALG = 0;
        const EXT2_LZRW3A_ALG = 1<<0;
        const EXT2_GZIP_ALG = 1<<1;
        const EXT2_BZIP2_ALG = 1<<2;
        const EXT2_LZO_ALG = 1<<3;
    }
}
