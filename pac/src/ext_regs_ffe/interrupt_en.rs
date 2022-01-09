#[doc = "Register `INTERRUPT_EN` reader"]
pub struct R(crate::R<INTERRUPT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_EN` writer"]
pub struct W(crate::W<INTERRUPT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTERRUPT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_MULT_WR_INTR_EN_A {
    #[doc = "0: Mask the interrupt event"]
    MASK = 0,
    #[doc = "1: Unmask the interrupt event"]
    UNMASK = 1,
}
impl From<SM_MULT_WR_INTR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SM_MULT_WR_INTR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM_MULT_WR_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub struct SM_MULT_WR_INTR_EN_R(crate::FieldReader<bool, SM_MULT_WR_INTR_EN_A>);
impl SM_MULT_WR_INTR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM_MULT_WR_INTR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_MULT_WR_INTR_EN_A {
        match self.bits {
            false => SM_MULT_WR_INTR_EN_A::MASK,
            true => SM_MULT_WR_INTR_EN_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == SM_MULT_WR_INTR_EN_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == SM_MULT_WR_INTR_EN_A::UNMASK
    }
}
impl core::ops::Deref for SM_MULT_WR_INTR_EN_R {
    type Target = crate::FieldReader<bool, SM_MULT_WR_INTR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_MULT_WR_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct SM_MULT_WR_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_MULT_WR_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM_MULT_WR_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SM_MULT_WR_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SM_MULT_WR_INTR_EN_A::UNMASK)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_OVERRUN_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `FFE0_OVERRUN_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_OVERRUN_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `FFE0_OVERRUN_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct FFE0_OVERRUN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_OVERRUN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_OVERRUN_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(FFE0_OVERRUN_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(FFE0_OVERRUN_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_SM1_OVERRUN_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `FFE0_SM1_OVERRUN_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_SM1_OVERRUN_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `FFE0_SM1_OVERRUN_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct FFE0_SM1_OVERRUN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_SM1_OVERRUN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_SM1_OVERRUN_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(FFE0_SM1_OVERRUN_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(FFE0_SM1_OVERRUN_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_SM0_OVERRUN_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `FFE0_SM0_OVERRUN_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_SM0_OVERRUN_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `FFE0_SM0_OVERRUN_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct FFE0_SM0_OVERRUN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_SM0_OVERRUN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_SM0_OVERRUN_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(FFE0_SM0_OVERRUN_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(FFE0_SM0_OVERRUN_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type I2C_MS_1_ERROR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `I2C_MS_1_ERROR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type I2C_MS_1_ERROR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `I2C_MS_1_ERROR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct I2C_MS_1_ERROR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MS_1_ERROR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_MS_1_ERROR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(I2C_MS_1_ERROR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(I2C_MS_1_ERROR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type I2C_MS_0_ERROR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `I2C_MS_0_ERROR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type I2C_MS_0_ERROR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `I2C_MS_0_ERROR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct I2C_MS_0_ERROR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MS_0_ERROR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_MS_0_ERROR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(I2C_MS_0_ERROR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(I2C_MS_0_ERROR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type CM_8K_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `CM_8k_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type CM_8K_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `CM_8k_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct CM_8K_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_8K_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_8K_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(CM_8K_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(CM_8K_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type DM0_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `DM0_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type DM0_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `DM0_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct DM0_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM0_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM0_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DM0_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DM0_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type DM1_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `DM1_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type DM1_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `DM1_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct DM1_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM1_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM1_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DM1_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DM1_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type SM0_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `SM0_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type SM0_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `SM0_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct SM0_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM0_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SM0_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SM0_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type SM1_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `SM1_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type SM1_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `SM1_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct SM1_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM1_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SM1_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SM1_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_BP_MATCH_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `FFE0_BP_MATCH_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type FFE0_BP_MATCH_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `FFE0_BP_MATCH_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct FFE0_BP_MATCH_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BP_MATCH_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_BP_MATCH_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(FFE0_BP_MATCH_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(FFE0_BP_MATCH_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type FFE1_OVERRUN_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `FFE1_OVERRUN_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type FFE1_OVERRUN_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `FFE1_OVERRUN_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct FFE1_OVERRUN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE1_OVERRUN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE1_OVERRUN_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(FFE1_OVERRUN_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(FFE1_OVERRUN_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type PKFB_OVF_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `PKFB_OVF_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type PKFB_OVF_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `PKFB_OVF_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct PKFB_OVF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKFB_OVF_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PKFB_OVF_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PKFB_OVF_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PKFB_OVF_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type SM0_BP_MATCH_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `SM0_BP_MATCH_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type SM0_BP_MATCH_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `SM0_BP_MATCH_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct SM0_BP_MATCH_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_BP_MATCH_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM0_BP_MATCH_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SM0_BP_MATCH_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SM0_BP_MATCH_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type SM1_BP_MATCH_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `SM1_BP_MATCH_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type SM1_BP_MATCH_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `SM1_BP_MATCH_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct SM1_BP_MATCH_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_BP_MATCH_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM1_BP_MATCH_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SM1_BP_MATCH_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SM1_BP_MATCH_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type SPI_MS_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `SPI_MS_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type SPI_MS_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `SPI_MS_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct SPI_MS_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MS_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_MS_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SPI_MS_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(SPI_MS_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type CM_2K_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `CM_2k_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type CM_2K_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `CM_2k_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct CM_2K_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_2K_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_2K_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(CM_2K_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(CM_2K_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type DM2_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `DM2_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type DM2_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `DM2_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct DM2_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM2_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM2_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DM2_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DM2_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type DM3_LP_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `DM3_LP_INTR_EN` reader - Set to enable the interrupt, leave cleared to mask"]
pub type DM3_LP_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `DM3_LP_INTR_EN` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct DM3_LP_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM3_LP_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM3_LP_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DM3_LP_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DM3_LP_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Set to enable the interrupt, leave cleared to mask"]
pub type AHBM_BUS_ERROR_INTR_EN_A = SM_MULT_WR_INTR_EN_A;
#[doc = "Field `ahbm_bus_error_intr_en` reader - Set to enable the interrupt, leave cleared to mask"]
pub type AHBM_BUS_ERROR_INTR_EN_R = SM_MULT_WR_INTR_EN_R;
#[doc = "Field `ahbm_bus_error_intr_en` writer - Set to enable the interrupt, leave cleared to mask"]
pub struct AHBM_BUS_ERROR_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_BUS_ERROR_INTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHBM_BUS_ERROR_INTR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AHBM_BUS_ERROR_INTR_EN_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AHBM_BUS_ERROR_INTR_EN_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm_mult_wr_intr_en(&self) -> SM_MULT_WR_INTR_EN_R {
        SM_MULT_WR_INTR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_overrun_en(&self) -> FFE0_OVERRUN_EN_R {
        FFE0_OVERRUN_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_sm1_overrun_en(&self) -> FFE0_SM1_OVERRUN_EN_R {
        FFE0_SM1_OVERRUN_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_sm0_overrun_en(&self) -> FFE0_SM0_OVERRUN_EN_R {
        FFE0_SM0_OVERRUN_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn i2c_ms_1_error_en(&self) -> I2C_MS_1_ERROR_EN_R {
        I2C_MS_1_ERROR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn i2c_ms_0_error_en(&self) -> I2C_MS_0_ERROR_EN_R {
        I2C_MS_0_ERROR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn cm_8k_lp_intr_en(&self) -> CM_8K_LP_INTR_EN_R {
        CM_8K_LP_INTR_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm0_lp_intr_en(&self) -> DM0_LP_INTR_EN_R {
        DM0_LP_INTR_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm1_lp_intr_en(&self) -> DM1_LP_INTR_EN_R {
        DM1_LP_INTR_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm0_lp_intr_en(&self) -> SM0_LP_INTR_EN_R {
        SM0_LP_INTR_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm1_lp_intr_en(&self) -> SM1_LP_INTR_EN_R {
        SM1_LP_INTR_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_bp_match_intr_en(&self) -> FFE0_BP_MATCH_INTR_EN_R {
        FFE0_BP_MATCH_INTR_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe1_overrun_en(&self) -> FFE1_OVERRUN_EN_R {
        FFE1_OVERRUN_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn pkfb_ovf_en(&self) -> PKFB_OVF_EN_R {
        PKFB_OVF_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm0_bp_match_intr_en(&self) -> SM0_BP_MATCH_INTR_EN_R {
        SM0_BP_MATCH_INTR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm1_bp_match_intr_en(&self) -> SM1_BP_MATCH_INTR_EN_R {
        SM1_BP_MATCH_INTR_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn spi_ms_intr_en(&self) -> SPI_MS_INTR_EN_R {
        SPI_MS_INTR_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn cm_2k_lp_intr_en(&self) -> CM_2K_LP_INTR_EN_R {
        CM_2K_LP_INTR_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm2_lp_intr_en(&self) -> DM2_LP_INTR_EN_R {
        DM2_LP_INTR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm3_lp_intr_en(&self) -> DM3_LP_INTR_EN_R {
        DM3_LP_INTR_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ahbm_bus_error_intr_en(&self) -> AHBM_BUS_ERROR_INTR_EN_R {
        AHBM_BUS_ERROR_INTR_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm_mult_wr_intr_en(&mut self) -> SM_MULT_WR_INTR_EN_W {
        SM_MULT_WR_INTR_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_overrun_en(&mut self) -> FFE0_OVERRUN_EN_W {
        FFE0_OVERRUN_EN_W { w: self }
    }
    #[doc = "Bit 4 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_sm1_overrun_en(&mut self) -> FFE0_SM1_OVERRUN_EN_W {
        FFE0_SM1_OVERRUN_EN_W { w: self }
    }
    #[doc = "Bit 5 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_sm0_overrun_en(&mut self) -> FFE0_SM0_OVERRUN_EN_W {
        FFE0_SM0_OVERRUN_EN_W { w: self }
    }
    #[doc = "Bit 6 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn i2c_ms_1_error_en(&mut self) -> I2C_MS_1_ERROR_EN_W {
        I2C_MS_1_ERROR_EN_W { w: self }
    }
    #[doc = "Bit 7 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn i2c_ms_0_error_en(&mut self) -> I2C_MS_0_ERROR_EN_W {
        I2C_MS_0_ERROR_EN_W { w: self }
    }
    #[doc = "Bit 8 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn cm_8k_lp_intr_en(&mut self) -> CM_8K_LP_INTR_EN_W {
        CM_8K_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 9 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm0_lp_intr_en(&mut self) -> DM0_LP_INTR_EN_W {
        DM0_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 10 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm1_lp_intr_en(&mut self) -> DM1_LP_INTR_EN_W {
        DM1_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 11 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm0_lp_intr_en(&mut self) -> SM0_LP_INTR_EN_W {
        SM0_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 12 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm1_lp_intr_en(&mut self) -> SM1_LP_INTR_EN_W {
        SM1_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 13 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe0_bp_match_intr_en(&mut self) -> FFE0_BP_MATCH_INTR_EN_W {
        FFE0_BP_MATCH_INTR_EN_W { w: self }
    }
    #[doc = "Bit 14 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ffe1_overrun_en(&mut self) -> FFE1_OVERRUN_EN_W {
        FFE1_OVERRUN_EN_W { w: self }
    }
    #[doc = "Bit 15 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn pkfb_ovf_en(&mut self) -> PKFB_OVF_EN_W {
        PKFB_OVF_EN_W { w: self }
    }
    #[doc = "Bit 16 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm0_bp_match_intr_en(&mut self) -> SM0_BP_MATCH_INTR_EN_W {
        SM0_BP_MATCH_INTR_EN_W { w: self }
    }
    #[doc = "Bit 17 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn sm1_bp_match_intr_en(&mut self) -> SM1_BP_MATCH_INTR_EN_W {
        SM1_BP_MATCH_INTR_EN_W { w: self }
    }
    #[doc = "Bit 18 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn spi_ms_intr_en(&mut self) -> SPI_MS_INTR_EN_W {
        SPI_MS_INTR_EN_W { w: self }
    }
    #[doc = "Bit 19 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn cm_2k_lp_intr_en(&mut self) -> CM_2K_LP_INTR_EN_W {
        CM_2K_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 20 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm2_lp_intr_en(&mut self) -> DM2_LP_INTR_EN_W {
        DM2_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 21 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn dm3_lp_intr_en(&mut self) -> DM3_LP_INTR_EN_W {
        DM3_LP_INTR_EN_W { w: self }
    }
    #[doc = "Bit 22 - Set to enable the interrupt, leave cleared to mask"]
    #[inline(always)]
    pub fn ahbm_bus_error_intr_en(&mut self) -> AHBM_BUS_ERROR_INTR_EN_W {
        AHBM_BUS_ERROR_INTR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the masking for different Flexible Fusion Engine interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_en](index.html) module"]
pub struct INTERRUPT_EN_SPEC;
impl crate::RegisterSpec for INTERRUPT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_en::R](R) reader structure"]
impl crate::Readable for INTERRUPT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_en::W](W) writer structure"]
impl crate::Writable for INTERRUPT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT_EN to value 0"]
impl crate::Resettable for INTERRUPT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
