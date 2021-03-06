#[doc = "Reader of register AP7816B_T0"]
pub type R = crate::R<u8, super::AP7816B_T0>;
#[doc = "Writer for register AP7816B_T0"]
pub type W = crate::W<u8, super::AP7816B_T0>;
#[doc = "Register AP7816B_T0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AP7816B_T0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADTI_L`"]
pub type ADTI_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADTI_L`"]
pub struct ADTI_L_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTI_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ATR Duration Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn adti_l(&self) -> ADTI_L_R {
        ADTI_L_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATR Duration Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn adti_l(&mut self) -> ADTI_L_W {
        ADTI_L_W { w: self }
    }
}
