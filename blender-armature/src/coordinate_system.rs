use crate::{BlenderArmature, Bone};

/// A coordinate system is used to make sense of coordinates.
///
/// Without knowing the coordinate system you wouldn't know if a value is meant for the
/// Y axis, or Z axis.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct CoordinateSystem {
    up: Axis,
    hand: Hand,
}

impl CoordinateSystem {
    #[allow(missing_docs)]
    pub fn new(up: Axis, hand: Hand) -> Self {
        CoordinateSystem { up, hand }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Represents the orientation of the coordinate system using the [right hand rule].
///
/// [right hand rule]: https://en.wikipedia.org/wiki/Right-hand_rule
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Hand {
    /// A right handed coordinate system
    Right,
    /// A left handed coordinate system
    Left,
}

/// Blender's coordinate system is Z up right handed
impl Default for CoordinateSystem {
    fn default() -> Self {
        CoordinateSystem {
            up: Axis::Z,
            hand: Hand::Right,
        }
    }
}

impl BlenderArmature {
    /// Shift around the data in the armature to a new coordinate system.
    ///
    /// For example, if the armature was previously Z up and we're switching to Y up
    ///  - the new +Y axis would be the old +Z axis
    ///  - the new +Z axis would be the old -Y axis
    pub fn change_coordinate_system(&mut self, system: CoordinateSystem) {
        if self.coordinate_system == system {
            return;
        }

        match (
            (self.coordinate_system.hand, self.coordinate_system.up),
            (system.hand, system.up),
        ) {
            ((Hand::Right, Axis::Z), (Hand::Right, Axis::Y)) => {
                for bone in self.inverse_bind_poses.iter_mut() {
                    *bone = dual_quat_z_up_right_to_y_up_right(*bone);
                }

                for (_action_name, action) in self.bone_space_actions.iter_mut() {
                    for (_bone_idx, keyframes) in action.keyframes_mut() {
                        for bone_keyframe in keyframes.iter_mut() {
                            let bone = bone_keyframe.bone();
                            bone_keyframe.set_bone(dual_quat_z_up_right_to_y_up_right(bone));
                        }
                    }
                }
            }
            _ => unimplemented!(),
        }

        self.coordinate_system = system;
    }
}

fn dual_quat_z_up_right_to_y_up_right(bone: Bone) -> Bone {
    match bone {
        Bone::Matrix(_) => unimplemented!(),
        Bone::DualQuat(mut dq) => {
            let rot_y = dq.real.j;
            let rot_z = dq.real.k;

            dq.real.j = rot_z;
            dq.real.k = -rot_y;

            let trans_y = dq.dual.j;
            let trans_z = dq.dual.k;

            dq.dual.j = trans_z;
            dq.dual.k = -trans_y;

            Bone::DualQuat(dq)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpolate::tests::dq_to_bone;
    use crate::test_util::{action_name, action_with_keyframes, BONE_IDX};
    use crate::{Action, BlenderArmature, BoneKeyframe, Keyframe};
    use std::collections::HashMap;

    /// Convert from the default Z-up right handed coordinate system to a Y-up right handed
    /// coordinate system.
    #[test]
    fn convert_dual_quaternions_z_up_right_to_y_up_right() {
        let mut arm = BlenderArmature::default();

        let expected_bone = dq_to_bone([0., 1., 3., -2., 4., 5., 7., -6.]);

        let bone = dq_to_bone([0., 1., 2., 3., 4., 5., 6., 7.]);
        arm.inverse_bind_poses = vec![bone.clone()];

        let keyframes = vec![BoneKeyframe::new(0, bone)];

        arm.bone_space_actions = action_with_keyframes(keyframes);

        arm.change_coordinate_system(CoordinateSystem::new(Axis::Y, Hand::Right));

        assert_eq!(&arm.inverse_bind_poses[0], &expected_bone);
        assert_eq!(
            &arm.bone_space_actions[&action_name()].bone_keyframes()[&BONE_IDX][0].bone(),
            &expected_bone
        );
    }

    /// If the armature is already using the coordinate system that we want to change to
    /// then nothing should change
    #[test]
    fn does_not_change_if_coordinate_system_same() {
        let mut arm = BlenderArmature::default();
        arm.change_coordinate_system(CoordinateSystem::new(Axis::Y, Hand::Right));

        let expected_bone = dq_to_bone([0., 1., 2., 3., 4., 5., 6., 7.]);

        let bone = dq_to_bone([0., 1., 2., 3., 4., 5., 6., 7.]);
        arm.inverse_bind_poses = vec![bone];

        let keyframes = vec![BoneKeyframe::new(0, bone)];

        arm.bone_space_actions = action_with_keyframes(keyframes);

        arm.change_coordinate_system(CoordinateSystem::new(Axis::Y, Hand::Right));

        assert_eq!(&arm.inverse_bind_poses[0], &expected_bone);
        assert_eq!(
            &arm.bone_space_actions[&action_name()].bone_keyframes()[&BONE_IDX][0].bone(),
            &expected_bone
        );
    }
}
