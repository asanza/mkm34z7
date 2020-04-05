#[doc = "Reader of register SSRS0"]
pub type R = crate::R<u8, super::SSRS0>;
#[doc = "Writer for register SSRS0"]
pub type W = crate::W<u8, super::SSRS0>;
#[doc = "Register SSRS0 `reset()`'s with value 0x82"]
impl crate::ResetValue for super::SSRS0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x82
    }
}
#[doc = "Sticky Low Leakage Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAKEUP_A {
    #[doc = "0: Reset not caused by wakeup source"]
    _0 = 0,
    #[doc = "1: Reset caused by wakeup source"]
    _1 = 1,
}
impl From<SWAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWAKEUP`"]
pub type SWAKEUP_R = crate::R<bool, SWAKEUP_A>;
impl SWAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAKEUP_A {
        match self.bits {
            false => SWAKEUP_A::_0,
            true => SWAKEUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAKEUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAKEUP_A::_1
    }
}
#[doc = "Write proxy for field `SWAKEUP`"]
pub struct SWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAKEUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWAKEUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWAKEUP_A::_0)
    }
    #[doc = "Reset caused by wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWAKEUP_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Sticky Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVD_A {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    _0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    _1 = 1,
}
impl From<SLVD_A> for bool {
    #[inline(always)]
    fn from(variant: SLVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVD`"]
pub type SLVD_R = crate::R<bool, SLVD_A>;
impl SLVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVD_A {
        match self.bits {
            false => SLVD_A::_0,
            true => SLVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLVD_A::_1
    }
}
#[doc = "Write proxy for field `SLVD`"]
pub struct SLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVD_A::_0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Sticky Loss-of-Lock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOL_A {
    #[doc = "0: Reset not caused by a loss of lock in the PLL"]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of lock in the PLL"]
    _1 = 1,
}
impl From<SLOL_A> for bool {
    #[inline(always)]
    fn from(variant: SLOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLOL`"]
pub type SLOL_R = crate::R<bool, SLOL_A>;
impl SLOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOL_A {
        match self.bits {
            false => SLOL_A::_0,
            true => SLOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLOL_A::_1
    }
}
#[doc = "Write proxy for field `SLOL`"]
pub struct SLOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by a loss of lock in the PLL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOL_A::_0)
    }
    #[doc = "Reset caused by a loss of lock in the PLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Sticky Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    _0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    _1 = 1,
}
impl From<SWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: SWDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWDOG`"]
pub type SWDOG_R = crate::R<bool, SWDOG_A>;
impl SWDOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDOG_A {
        match self.bits {
            false => SWDOG_A::_0,
            true => SWDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWDOG_A::_1
    }
}
#[doc = "Write proxy for field `SWDOG`"]
pub struct SWDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWDOG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWDOG_A::_0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWDOG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Sticky External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    _0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    _1 = 1,
}
impl From<SPIN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIN`"]
pub type SPIN_R = crate::R<bool, SPIN_A>;
impl SPIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIN_A {
        match self.bits {
            false => SPIN_A::_0,
            true => SPIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIN_A::_1
    }
}
#[doc = "Write proxy for field `SPIN`"]
pub struct SPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIN_A::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Sticky Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOR_A {
    #[doc = "0: Reset not caused by POR"]
    _0 = 0,
    #[doc = "1: Reset caused by POR"]
    _1 = 1,
}
impl From<SPOR_A> for bool {
    #[inline(always)]
    fn from(variant: SPOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPOR`"]
pub type SPOR_R = crate::R<bool, SPOR_A>;
impl SPOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOR_A {
        match self.bits {
            false => SPOR_A::_0,
            true => SPOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPOR_A::_1
    }
}
#[doc = "Write proxy for field `SPOR`"]
pub struct SPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPOR_A::_0)
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPOR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn swakeup(&self) -> SWAKEUP_R {
        SWAKEUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&self) -> SLVD_R {
        SLVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&self) -> SLOL_R {
        SLOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&self) -> SWDOG_R {
        SWDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&self) -> SPIN_R {
        SPIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&self) -> SPOR_R {
        SPOR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn swakeup(&mut self) -> SWAKEUP_W {
        SWAKEUP_W { w: self }
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&mut self) -> SLVD_W {
        SLVD_W { w: self }
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&mut self) -> SLOL_W {
        SLOL_W { w: self }
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&mut self) -> SWDOG_W {
        SWDOG_W { w: self }
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&mut self) -> SPIN_W {
        SPIN_W { w: self }
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&mut self) -> SPOR_W {
        SPOR_W { w: self }
    }
}