#[doc = "Reader of register SAR0"]
pub type R = crate::R<u32, super::SAR0>;
#[doc = "Writer for register SAR0"]
pub type W = crate::W<u32, super::SAR0>;
#[doc = "Register SAR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR`"]
pub type SAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAR`"]
pub struct SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    pub fn sar(&mut self) -> SAR_W {
        SAR_W { w: self }
    }
}
