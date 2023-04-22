use std::cmp::Ordering;

use crate::{aabb::surrounding_box, hitable::HitRecord, ray::Ray};

use super::{aabb::AABB, hitable::Hitable};

/// 枚举：节点的内容
/// Branch：分叉节点，左右子树信息
/// Leaf：叶子节点，实际的渲染对象
enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hitable>),
}

///
pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    /// hitlist是sphere的列表
    pub fn new(mut hitlist: Vec<Box<dyn Hitable>>, time0: f64, time1: f64) -> Self {
        // 递归二分处理所有hitlist中的对象

        // 辅助函数1：创建一个闭包，用于在给定轴上比较两个bbox
        // 比较该轴上的最大值与最小值之和
        fn box_compare(
            time0: f64,
            time1: f64,
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hitable>, &Box<dyn Hitable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min()[axis] + a.max()[axis];
                    let bc = b.min()[axis] + b.max()[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
        }

        // 辅助函数2：给出hitlist在选定轴上的范围
        fn axis_range(hitlist: &Vec<Box<dyn Hitable>>, time0: f64, time1: f64, axis: usize) -> f64 {
            let (min, max) = hitlist
                .iter()
                .fold((f64::MAX, f64::MIN), |(bmin, bmax), bhit| {
                    if let Some(bbox) = bhit.bounding_box(time0, time1) {
                        (bmin.min(bbox.min()[axis]), bmax.max(bbox.max()[axis]))
                    } else {
                        (bmin, bmax)
                    }
                });
            max - min
        }

        // 找到分布最广的轴
        let (axis, _) = (0..3)
            .map(|a| (a, axis_range(&hitlist, time0, time1, a)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        // 在该轴上给hitlist排序
        hitlist.sort_unstable_by(box_compare(time0, time1, axis));

        // 处理最后得到的hitlist，构建一个BVH树，叶子节点是object的包围盒->object
        let len = hitlist.len();
        match len {
            0 => panic!("no elements in the world"),
            1 => {
                let leaf = hitlist.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    return BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    };
                } else {
                    panic!("no bounding box in bvh node")
                };
            }
            _ => {
                let right = BVH::new(hitlist.drain(len / 2..).collect(), time0, time1);
                let left = BVH::new(hitlist, time0, time1);
                let bbox = surrounding_box(&left.bbox, &right.bbox);
                return BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox: bbox,
                };
            }
        }
    }
}

impl Hitable for BVH {
    fn hit(&self, r: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        // 递归，直到命中最近的子节点
        if self.bbox.hit(r, t_min, t_max) {
            // 与包围盒相交
            match &self.tree {
                BVHNode::Branch { left, right } => {
                    // left是较近的，如果命中，更新t_max
                    let left = left.hit(r, t_min, t_max);
                    if let Some(l) = &left {
                        t_max = l.time()
                    };
                    // 左右有重叠，更新t_max仍与右节点相交，右节点是最近的
                    let right = right.hit(r, t_min, t_max);
                    if right.is_some() {
                        right
                    } else {
                        left
                    }
                }
                BVHNode::Leaf(leaf) => leaf.hit(r, t_min, t_max),
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time11: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}
