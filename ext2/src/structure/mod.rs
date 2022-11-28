pub mod superblock;
pub mod blockgroupdescriptor;
pub mod bitmap;
pub mod inode;

fn calc_block_group(inode: usize, s_inode_per_group: usize) -> usize {
    /*
    calculate which group is inode located 

    inode: number of inode
    s_inode_per_group: inode per group recode in superblock
    */
    (inode - 1) / s_inode_per_group
}

fn calc_inode_index(inode: usize, s_inode_per_group: usize) -> usize {
    /*
    calculate the inode index in the group

    inode: number of inode
    s_inode_per_group: inode per groun recode in superblock
    */
    (inode - 1) % s_inode_per_group
}