#[doc = "Register `M4_SRAM_SW_PD` reader"]
pub struct R(crate::R<M4_SRAM_SW_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_SRAM_SW_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_SRAM_SW_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_SRAM_SW_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4_SRAM_SW_PD` writer"]
pub struct W(crate::W<M4_SRAM_SW_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4_SRAM_SW_PD_SPEC>;
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
impl From<crate::W<M4_SRAM_SW_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4_SRAM_SW_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4S0_SOFTWARE_PD_A {
    #[doc = "1: Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    POWER_DOWN = 1,
}
impl From<M4S0_SOFTWARE_PD_A> for bool {
    #[inline(always)]
    fn from(variant: M4S0_SOFTWARE_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M4S0_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S0_SOFTWARE_PD_R(crate::FieldReader<bool, M4S0_SOFTWARE_PD_A>);
impl M4S0_SOFTWARE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S0_SOFTWARE_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M4S0_SOFTWARE_PD_A> {
        match self.bits {
            true => Some(M4S0_SOFTWARE_PD_A::POWER_DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == M4S0_SOFTWARE_PD_A::POWER_DOWN
    }
}
impl core::ops::Deref for M4S0_SOFTWARE_PD_R {
    type Target = crate::FieldReader<bool, M4S0_SOFTWARE_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S0_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S0_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S0_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S0_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S0_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S1_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S1_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S1_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S1_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S1_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S1_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S1_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S1_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S2_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S2_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S2_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S2_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S2_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S2_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S2_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S2_SOFTWARE_PD_A::POWER_DOWN)
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
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S3_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S3_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S3_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S3_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S3_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S3_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S3_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S3_SOFTWARE_PD_A::POWER_DOWN)
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S4_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S4_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S4_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S4_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S4_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S4_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S4_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S4_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S5_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S5_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S5_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S5_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S5_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S5_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S5_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S5_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S6_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S6_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S6_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S6_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S6_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S6_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S6_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S6_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S7_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S7_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S7_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S7_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S7_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S7_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S7_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S7_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S8_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S8_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S8_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S8_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S8_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S8_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S8_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S8_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S9_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S9_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S9_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S9_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S9_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S9_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S9_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S9_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S10_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S10_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S10_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S10_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S10_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S10_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S10_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S10_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S11_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S11_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S11_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S11_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S11_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S11_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S11_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S11_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S12_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S12_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S12_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S12_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S12_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S12_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S12_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S12_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S13_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S13_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S13_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S13_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S13_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S13_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S13_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S13_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S14_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S14_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S14_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S14_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S14_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S14_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S14_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S14_SOFTWARE_PD_A::POWER_DOWN)
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
#[doc = "Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S15_SOFTWARE_PD_A = M4S0_SOFTWARE_PD_A;
#[doc = "Field `M4S15_Software_PD` reader - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub type M4S15_SOFTWARE_PD_R = M4S0_SOFTWARE_PD_R;
#[doc = "Field `M4S15_Software_PD` writer - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
pub struct M4S15_SOFTWARE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4S15_SOFTWARE_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4S15_SOFTWARE_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Put the power domain to power saving mode according to configuration setting, HW will clear it once power down sequence is finished."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(M4S15_SOFTWARE_PD_A::POWER_DOWN)
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
impl R {
    #[doc = "Bit 0 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s0_software_pd(&self) -> M4S0_SOFTWARE_PD_R {
        M4S0_SOFTWARE_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s1_software_pd(&self) -> M4S1_SOFTWARE_PD_R {
        M4S1_SOFTWARE_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s2_software_pd(&self) -> M4S2_SOFTWARE_PD_R {
        M4S2_SOFTWARE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s3_software_pd(&self) -> M4S3_SOFTWARE_PD_R {
        M4S3_SOFTWARE_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s4_software_pd(&self) -> M4S4_SOFTWARE_PD_R {
        M4S4_SOFTWARE_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s5_software_pd(&self) -> M4S5_SOFTWARE_PD_R {
        M4S5_SOFTWARE_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s6_software_pd(&self) -> M4S6_SOFTWARE_PD_R {
        M4S6_SOFTWARE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s7_software_pd(&self) -> M4S7_SOFTWARE_PD_R {
        M4S7_SOFTWARE_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s8_software_pd(&self) -> M4S8_SOFTWARE_PD_R {
        M4S8_SOFTWARE_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s9_software_pd(&self) -> M4S9_SOFTWARE_PD_R {
        M4S9_SOFTWARE_PD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s10_software_pd(&self) -> M4S10_SOFTWARE_PD_R {
        M4S10_SOFTWARE_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s11_software_pd(&self) -> M4S11_SOFTWARE_PD_R {
        M4S11_SOFTWARE_PD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s12_software_pd(&self) -> M4S12_SOFTWARE_PD_R {
        M4S12_SOFTWARE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s13_software_pd(&self) -> M4S13_SOFTWARE_PD_R {
        M4S13_SOFTWARE_PD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s14_software_pd(&self) -> M4S14_SOFTWARE_PD_R {
        M4S14_SOFTWARE_PD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s15_software_pd(&self) -> M4S15_SOFTWARE_PD_R {
        M4S15_SOFTWARE_PD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s0_software_pd(&mut self) -> M4S0_SOFTWARE_PD_W {
        M4S0_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 1 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s1_software_pd(&mut self) -> M4S1_SOFTWARE_PD_W {
        M4S1_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 2 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s2_software_pd(&mut self) -> M4S2_SOFTWARE_PD_W {
        M4S2_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 3 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s3_software_pd(&mut self) -> M4S3_SOFTWARE_PD_W {
        M4S3_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 4 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s4_software_pd(&mut self) -> M4S4_SOFTWARE_PD_W {
        M4S4_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 5 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s5_software_pd(&mut self) -> M4S5_SOFTWARE_PD_W {
        M4S5_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 6 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s6_software_pd(&mut self) -> M4S6_SOFTWARE_PD_W {
        M4S6_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 7 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s7_software_pd(&mut self) -> M4S7_SOFTWARE_PD_W {
        M4S7_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 8 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s8_software_pd(&mut self) -> M4S8_SOFTWARE_PD_W {
        M4S8_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 9 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s9_software_pd(&mut self) -> M4S9_SOFTWARE_PD_W {
        M4S9_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 10 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s10_software_pd(&mut self) -> M4S10_SOFTWARE_PD_W {
        M4S10_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 11 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s11_software_pd(&mut self) -> M4S11_SOFTWARE_PD_W {
        M4S11_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 12 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s12_software_pd(&mut self) -> M4S12_SOFTWARE_PD_W {
        M4S12_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 13 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s13_software_pd(&mut self) -> M4S13_SOFTWARE_PD_W {
        M4S13_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 14 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s14_software_pd(&mut self) -> M4S14_SOFTWARE_PD_W {
        M4S14_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Bit 15 - Set to trigger a power down event. Bit is cleared after power down sequence finishes"]
    #[inline(always)]
    pub fn m4s15_software_pd(&mut self) -> M4S15_SOFTWARE_PD_W {
        M4S15_SOFTWARE_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for triggering power-down events in M4 SRAM power domains. (RWHC)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_sram_sw_pd](index.html) module"]
pub struct M4_SRAM_SW_PD_SPEC;
impl crate::RegisterSpec for M4_SRAM_SW_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_sram_sw_pd::R](R) reader structure"]
impl crate::Readable for M4_SRAM_SW_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4_sram_sw_pd::W](W) writer structure"]
impl crate::Writable for M4_SRAM_SW_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4_SRAM_SW_PD to value 0"]
impl crate::Resettable for M4_SRAM_SW_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
