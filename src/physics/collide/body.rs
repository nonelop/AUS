use crate::body::Body;
use crate::physics::collide::CollisionInfo;

pub trait CollidableBody: Body {
    fn collide(&mut self, with: &mut dyn Body, info: CollisionInfo) {

        let self_mass = self.get_mass();
        let with_mass = with.get_mass();

        let inv_self_mass = if self_mass <= 0.0 {0.0} else {1.0 / self_mass};
        let inv_with_mass = if with_mass <= 0.0 {0.0} else {1.0 / with_mass};
        let inv_total_mass = inv_self_mass + inv_with_mass;

        if inv_total_mass <= 0.0 {
            return;
        }

        self.set_position(self.get_position() + info.normal * (info.depth * inv_self_mass / inv_total_mass));
        with.set_position(with.get_position() - info.normal * (info.depth * inv_with_mass / inv_total_mass));

        let self_velocity = self.get_velocity();
        let with_velocity = with.get_velocity();

        let v_rel = with_velocity - self_velocity;
        let v_rel_n = v_rel.dot(info.normal);

        if v_rel_n <= 0.0 {
            return;
        }

        let restitution = 1.0;

        let j = (1.0 + restitution) * v_rel_n / inv_total_mass;

        self.set_velocity(self_velocity + info.normal * (j * inv_self_mass));
        with.set_velocity(with_velocity - info.normal * (j * inv_with_mass));
    }
}