use bevy::{
	prelude::{Component, Quat, Vec3},
	reflect::Reflect,
};
use lightyear::prelude::{client::InterpFn, Message};
use serde::{Deserialize, Serialize};

#[derive(
	Component,
	Message,
	Serialize,
	Deserialize,
	Clone,
	Debug,
	PartialEq,
	Default,
	Reflect,
)]
pub struct PlayerPose {
	/// The root of the avatar. Everything else is relative to this.
	pub root: Isometry,
	pub head: Isometry,
	pub hand_l: Isometry,
	pub hand_r: Isometry,
}

/// An isometry is like a `Transform`, but without scale.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
pub struct Isometry {
	pub trans: Vec3,
	pub rot: Quat,
}

/// Interpolates between two [`PlayerPose`]s.
pub struct PlayerPoseInterp;

impl InterpFn<PlayerPose> for PlayerPoseInterp {
	fn lerp(mut start: PlayerPose, other: PlayerPose, t: f32) -> PlayerPose {
		/// Mutates `start` for every specified field.
		macro_rules! call_interp {
			($start:ident, $other:ident, $t:ident, [$($field:ident),+]) => {
                $(
                    $start.$field = IsometryInterp::lerp($start.$field, $other.$field, $t);
                )+
            };
		}
		call_interp!(start, other, t, [root, head, hand_l, hand_r]);

		start
	}
}

/// Interpolates between two [`Isometry`]s.
struct IsometryInterp;

impl InterpFn<Isometry> for IsometryInterp {
	fn lerp(mut start: Isometry, other: Isometry, t: f32) -> Isometry {
		start.trans = start.trans.lerp(other.trans, t);
		start.rot = start.rot.lerp(other.rot, t);

		start
	}
}
