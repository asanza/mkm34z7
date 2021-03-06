#[doc = "Writer for register CAU_AESC_CA5"]
pub type W = crate::W<u32, super::CAU_AESC_CA5>;
#[doc = "Register CAU_AESC_CA5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAU_AESC_CA5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CA5`"]
pub struct CA5_W<'a> {
    w: &'a mut W,
}
impl<'a> CA5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CA5"]
    #[inline(always)]
    pub fn ca5(&mut self) -> CA5_W {
        CA5_W { w: self }
    }
}
