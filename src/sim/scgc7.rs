#[doc = "Reader of register SCGC7"]
pub type R = crate::R<u32, super::SCGC7>;
#[doc = "Writer for register SCGC7"]
pub type W = crate::W<u32, super::SCGC7>;
#[doc = "Register SCGC7 `reset()`'s with value 0x03"]
impl crate::ResetValue for super::SCGC7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "MPU Clock Gate control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPU_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<MPU_A> for bool {
    #[inline(always)]
    fn from(variant: MPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPU`"]
pub type MPU_R = crate::R<bool, MPU_A>;
impl MPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPU_A {
        match self.bits {
            false => MPU_A::_0,
            true => MPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPU_A::_1
    }
}
#[doc = "Write proxy for field `MPU`"]
pub struct MPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPU_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPU_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "DMA Clock Gate control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMA_A::_1
    }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "CAU Clock Gate control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAU_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CAU_A> for bool {
    #[inline(always)]
    fn from(variant: CAU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAU`"]
pub type CAU_R = crate::R<bool, CAU_A>;
impl CAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAU_A {
        match self.bits {
            false => CAU_A::_0,
            true => CAU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CAU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CAU_A::_1
    }
}
#[doc = "Write proxy for field `CAU`"]
pub struct CAU_W<'a> {
    w: &'a mut W,
}
impl<'a> CAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAU_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAU_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MPU Clock Gate control."]
    #[inline(always)]
    pub fn mpu(&self) -> MPU_R {
        MPU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Clock Gate control."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAU Clock Gate control."]
    #[inline(always)]
    pub fn cau(&self) -> CAU_R {
        CAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Clock Gate control."]
    #[inline(always)]
    pub fn mpu(&mut self) -> MPU_W {
        MPU_W { w: self }
    }
    #[doc = "Bit 1 - DMA Clock Gate control."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 2 - CAU Clock Gate control."]
    #[inline(always)]
    pub fn cau(&mut self) -> CAU_W {
        CAU_W { w: self }
    }
}
