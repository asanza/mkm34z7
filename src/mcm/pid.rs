#[doc = "Reader of register PID"]
pub type R = crate::R<u32, super::PID>;
#[doc = "Writer for register PID"]
pub type W = crate::W<u32, super::PID>;
#[doc = "Register PID `reset()`'s with value 0"]
impl crate::ResetValue for super::PID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "M0_PID For MPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: Reserved for privileged secure tasks"]
    _0 = 0,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, PID_A>;
impl PID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PID_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PID_A::_0
    }
}
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reserved for privileged secure tasks"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - M0_PID For MPU"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - M0_PID For MPU"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
}
