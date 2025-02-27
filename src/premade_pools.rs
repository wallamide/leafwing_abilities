//! Convenient premade resource [`Pool`] types to get you started.
//!
//! These can be annoying due to orphan rules that prevent you from implementing your own methods,
//! so feel free to copy-paste them (without attribution) into your own source to make new variants.

use crate::pool::{MaxPoolLessThanZero, Pool};
use bevy::prelude::{Component, Resource};
use core::ops::{Div, Mul};
use derive_more::{Add, AddAssign, Sub, SubAssign};

/// A premade resource pool for life (aka health, hit points or HP).
pub mod life {
    use super::*;

    /// The amount of life available to a unit.
    /// If they lose it all, they die or pass out.
    ///
    /// This is intended to be stored as a component on each entity.
    #[derive(Debug, Clone, PartialEq, Component, Resource)]
    pub struct LifePool {
        /// The current life.
        current: Life,
        /// The maximum life that can be stored.
        max: Life,
        /// The amount of life regenerated per second.
        pub regen_per_second: Life,
    }

    /// A quantity of life, used to modify a [`LifePool`].
    ///
    /// This can be used for damage computations, life regeneration, healing and so on.
    #[derive(
        Debug, Clone, Copy, PartialEq, PartialOrd, Default, Add, Sub, AddAssign, SubAssign,
    )]
    pub struct Life(pub f32);

    impl Mul<f32> for Life {
        type Output = Life;

        fn mul(self, rhs: f32) -> Life {
            Life(self.0 * rhs)
        }
    }

    impl Mul<Life> for f32 {
        type Output = Life;

        fn mul(self, rhs: Life) -> Life {
            Life(self * rhs.0)
        }
    }

    impl Div<f32> for Life {
        type Output = Life;

        fn div(self, rhs: f32) -> Life {
            Life(self.0 / rhs)
        }
    }

    impl Pool for LifePool {
        type Quantity = Life;
        const ZERO: Life = Life(0.);

        fn new(
            current: Self::Quantity,
            max: Self::Quantity,
            regen_per_second: Self::Quantity,
        ) -> Self {
            LifePool {
                current,
                max,
                regen_per_second,
            }
        }

        fn current(&self) -> Self::Quantity {
            self.current
        }

        fn set_current(&mut self, new_quantity: Self::Quantity) -> Self::Quantity {
            let actual_value = Life(new_quantity.0.clamp(0., self.max.0));
            self.current = actual_value;
            self.current
        }

        fn max(&self) -> Self::Quantity {
            self.max
        }

        fn set_max(&mut self, new_max: Self::Quantity) -> Result<(), MaxPoolLessThanZero> {
            if new_max < Self::ZERO {
                Err(MaxPoolLessThanZero)
            } else {
                self.max = new_max;
                self.set_current(self.current);
                Ok(())
            }
        }

        fn regen_per_second(&self) -> Self::Quantity {
            self.regen_per_second
        }

        fn set_regen_per_second(&mut self, new_regen_per_second: Self::Quantity) {
            self.regen_per_second = new_regen_per_second;
        }
    }
}

/// A premade resource pool for mana (aka MP).
pub mod mana {
    use super::*;

    /// The amount of mana available to a unit.
    /// Units must spend mana to cast spells according to their [`AbilityCosts<A, Mana>`](crate::pool::AbilityCosts) component.
    ///
    /// This is intended to be stored as a component on each entity.
    #[derive(Debug, Clone, PartialEq, Component, Resource)]
    pub struct ManaPool {
        /// The current mana.
        current: Mana,
        /// The maximum mana that can be stored.
        max: Mana,
        /// The amount of mana regenerated per second.
        pub regen_per_second: Mana,
    }

    /// A quantity of mana, used to modify a [`ManaPool`].
    ///
    /// This can be used for ability costs, mana regeneration and so on.
    #[derive(
        Debug, Clone, Copy, PartialEq, PartialOrd, Default, Add, Sub, AddAssign, SubAssign,
    )]
    pub struct Mana(pub f32);

    impl Mul<f32> for Mana {
        type Output = Mana;

        fn mul(self, rhs: f32) -> Mana {
            Mana(self.0 * rhs)
        }
    }

    impl Mul<Mana> for f32 {
        type Output = Mana;

        fn mul(self, rhs: Mana) -> Mana {
            Mana(self * rhs.0)
        }
    }

    impl Div<f32> for Mana {
        type Output = Mana;

        fn div(self, rhs: f32) -> Mana {
            Mana(self.0 / rhs)
        }
    }

    impl Pool for ManaPool {
        type Quantity = Mana;
        const ZERO: Mana = Mana(0.);

        fn new(
            current: Self::Quantity,
            max: Self::Quantity,
            regen_per_second: Self::Quantity,
        ) -> Self {
            ManaPool {
                current,
                max,
                regen_per_second,
            }
        }

        fn current(&self) -> Self::Quantity {
            self.current
        }

        fn set_current(&mut self, new_quantity: Self::Quantity) -> Self::Quantity {
            let actual_value = Mana(new_quantity.0.clamp(0., self.max.0));
            self.current = actual_value;
            self.current
        }

        fn max(&self) -> Self::Quantity {
            self.max
        }

        fn set_max(&mut self, new_max: Self::Quantity) -> Result<(), MaxPoolLessThanZero> {
            if new_max < Self::ZERO {
                Err(MaxPoolLessThanZero)
            } else {
                self.max = new_max;
                self.set_current(self.current);
                Ok(())
            }
        }

        fn regen_per_second(&self) -> Self::Quantity {
            self.regen_per_second
        }

        fn set_regen_per_second(&mut self, new_regen_per_second: Self::Quantity) {
            self.regen_per_second = new_regen_per_second;
        }
    }
}
