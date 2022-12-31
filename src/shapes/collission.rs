//! Contains the Collide trait, which is used for collission
//! detection between objects. All collider types implement the Collide trait.

use super::circle::CircleCollider;
use super::rect::RectangleCollider;
use cgmath::InnerSpace;

use std::cmp::Ordering;

/// Types implementing this trait can collide with other objects
pub trait Collide<Other = Self> {
    /// Checks if the object is colliding with the other object
    fn is_colliding_with(&self, other: &Other) -> bool;
}

impl Collide for RectangleCollider {
    fn is_colliding_with(&self, other: &Self) -> bool {
        // https://www.gamedev.net/tutorials/_/technical/game-programming/2d-rotated-rectangle-collision-r2604/

        // Use Separating Axis Theorem
        let edges = self.edges();
        let other_edges = other.edges();

        // Calculate the 4 axes perpendicular to the edges, 2 for each rectangle
        let axis1 = edges.top_right - edges.top_left;
        let axis2 = edges.top_right - edges.bottom_right;
        let axis3 = other_edges.top_right - other_edges.top_left;
        let axis4 = other_edges.top_right - other_edges.bottom_right;
        let axes = [axis1, axis2, axis3, axis4];

        // project edges onto axes
        // TODO: Check this code. There might be a bug here
        for axis in axes {
            let projected_edges = edges
                .iter()
                .map(|edge| edge.dot(axis) / (axis.dot(axis)) * axis);
            let projected_other_edges = other_edges
                .iter()
                .map(|edge| edge.dot(axis) / (axis.dot(axis)) * axis);

            let dot_products = edges.iter().map(|edge| edge.dot(axis));
            let min = dot_products
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            let max = dot_products
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let other_dot_products = other_edges.iter().map(|edge| edge.dot(axis));
            let other_min = other_dot_products
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            let other_max = other_dot_products
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            // check for overlap. If even one axis have no overlap,
            // then there is no collission
            if !(other_min <= max && other_max >= min) {
                return false;
            }
        }

        // If every axis had an overlap, then there is a collission
        return true;
    }
}

impl Collide<CircleCollider> for RectangleCollider {
    fn is_colliding_with(&self, other: &CircleCollider) -> bool {
        //TODO: Account for rect rotation
        let displacement = self.position - other.center;
        // let displacement = self.edges().top_right - other.center;
        let dx = displacement.x.abs() - self.size.x / 2.0;
        let dy = displacement.y.abs() - self.size.y / 2.0;

        println!("dx: {dx}, dy: {dy}");

        if dx > other.radius || dy > other.radius {
            return false;
        }

        if dx <= 0.0 || dy <= 0.0 {
            return true;
        }

        return dx * dx + dy * dy <= other.radius * other.radius;
    }
}

impl Collide for CircleCollider {
    fn is_colliding_with(&self, other: &Self) -> bool {
        let displacment = other.center - self.center;
        let distance = displacment.magnitude();

        return distance < self.radius + other.radius;
    }
}

impl Collide<RectangleCollider> for CircleCollider {
    fn is_colliding_with(&self, other: &RectangleCollider) -> bool {
        other.is_colliding_with(self)
    }
}
