#![no_std]

#[macro_export]
macro_rules! ebits {
        (
                $(#[$outer:meta])*
                $vis:vis struct $Name:ident: $T:ty {
                        $(
                                $(#[$inner:meta])*
                                const $Flag:ident = $Val:expr;
                        )*
                }
        ) => {
                $(#[$outer])*
                #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[repr(transparent)]
                $vis struct $Name(pub $T);

                impl $Name {
                        $(
                                $(#[$inner])*
                                pub const $Flag: Self = Self($Val);
                        )*

                        pub const NONE: Self = Self(0);

                        /// Returns a value containing every single defined flag.
                        pub const ALL: Self = Self(
                                0 $( | ($Val) )*
                        );

                        #[inline]
                        pub const fn bits(&self) -> $T {
                                self.0
                        }
			#[inline]
			pub const fn with(self, other: Self) -> Self {
				let bits = self.bits() | other.bits();
				Self(bits)
			}

			#[inline]
			pub const fn without(self, other: Self) -> Self {
				let oppose_bits = !other.bits();
				let bits = self.bits() | oppose_bits;
				Self(bits)
			}

                        #[inline]
                        pub const fn insert(&mut self, other: Self) {
                                self.0 |= other.bits(); // Fixed from &= to |=
                        }

                        #[inline]
                        pub const fn remove(&mut self, other: Self) {
                                let oppose = !other.bits();
                                self.0 &= oppose; // Fixed missing semicolon
                        }

                        #[inline]
                        pub const fn contains(&self, other: Self) -> bool {
                                (self.0 & other.0) == other.0
                        }
                }

		impl core::ops::BitOrAssign for $Name {
                        #[inline]
                        fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
                }

                impl core::ops::BitAndAssign for $Name {
                        #[inline]
                        fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
                }

                impl core::ops::BitXorAssign for $Name {
                        #[inline]
                        fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
                }

                impl core::ops::BitOr for $Name {
                        type Output = Self;
                        #[inline]
                        fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
                }

                impl core::ops::BitAnd for $Name {
                        type Output = Self;
                        #[inline]
                        fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
                }

                impl core::ops::BitXor for $Name {
                        type Output = Self;
                        #[inline]
                        fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
                }

                impl core::ops::Not for $Name {
                        type Output = Self;
                        #[inline]
                        fn not(self) -> Self { Self(!self.0) }
                }

                impl core::fmt::Debug for $Name {
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                                let mut first = true;
                                let mut accumulated = 0;

                                $(
                                        // Wrap $Val in parentheses to safely handle shifts (e.g., 1 << 0)
                                        let val = $Val;
                                        if val != 0 && (val & (val - 1)) == 0 {
                                                if self.contains(Self::$Flag) {
                                                        if !first { f.write_str(" | ")?; }
                                                        f.write_str(stringify!($Flag))?;
                                                        first = false;
                                                        accumulated |= val;
                                                }
                                        }
                                )*

                                let extra = self.0 & !accumulated;
                                if extra != 0 {
                                        if !first { f.write_str(" | ")?; }
                                        core::write!(f, "{:#X}", extra)?;
                                } else if first {
                                        f.write_str("NONE")?;
                                }

                                Ok(())
                        }
                }
        };
}
