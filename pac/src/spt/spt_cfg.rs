#[doc = "Register `SPT_CFG` reader"]
pub struct R(crate::R<SPT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPT_CFG` writer"]
pub struct W(crate::W<SPT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPT_CFG_SPEC>;
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
impl From<crate::W<SPT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field to enable the timer/counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPT_EN_A {
    #[doc = "0: Except 30 bits Timer, all the counter will be reset to 0."]
    DISABLE = 0,
    #[doc = "1: Turn on Counter/Timer"]
    ENABLE = 1,
}
impl From<SPT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPT_EN` reader - Field to enable the timer/counter"]
pub struct SPT_EN_R(crate::FieldReader<bool, SPT_EN_A>);
impl SPT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPT_EN_A {
        match self.bits {
            false => SPT_EN_A::DISABLE,
            true => SPT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SPT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SPT_EN_A::ENABLE
    }
}
impl core::ops::Deref for SPT_EN_R {
    type Target = crate::FieldReader<bool, SPT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPT_EN` writer - Field to enable the timer/counter"]
pub struct SPT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Except 30 bits Timer, all the counter will be reset to 0."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPT_EN_A::DISABLE)
    }
    #[doc = "Turn on Counter/Timer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPT_EN_A::ENABLE)
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
#[doc = "Select the clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: Clock source is 32KHz"]
    CLOCK_32KHZ = 0,
    #[doc = "1: Clock source is 16KHz"]
    CLOCK_16KHZ = 1,
}
impl From<CLK_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SRC_SEL` reader - Select the clock source"]
pub struct CLK_SRC_SEL_R(crate::FieldReader<bool, CLK_SRC_SEL_A>);
impl CLK_SRC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SRC_SEL_A {
        match self.bits {
            false => CLK_SRC_SEL_A::CLOCK_32KHZ,
            true => CLK_SRC_SEL_A::CLOCK_16KHZ,
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_32KHZ`"]
    #[inline(always)]
    pub fn is_clock_32khz(&self) -> bool {
        **self == CLK_SRC_SEL_A::CLOCK_32KHZ
    }
    #[doc = "Checks if the value of the field is `CLOCK_16KHZ`"]
    #[inline(always)]
    pub fn is_clock_16khz(&self) -> bool {
        **self == CLK_SRC_SEL_A::CLOCK_16KHZ
    }
}
impl core::ops::Deref for CLK_SRC_SEL_R {
    type Target = crate::FieldReader<bool, CLK_SRC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SRC_SEL` writer - Select the clock source"]
pub struct CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SRC_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source is 32KHz"]
    #[inline(always)]
    pub fn clock_32khz(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLOCK_32KHZ)
    }
    #[doc = "Clock source is 16KHz"]
    #[inline(always)]
    pub fn clock_16khz(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLOCK_16KHZ)
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
#[doc = "Activate or deactive interrupt trigger signal 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_MASK_N_0_A {
    #[doc = "0: Unmask the interrupt"]
    UNMASK = 0,
    #[doc = "1: Mask the interrupt"]
    MASK = 1,
}
impl From<INT_MASK_N_0_A> for bool {
    #[inline(always)]
    fn from(variant: INT_MASK_N_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_MASK_N_0` reader - Activate or deactive interrupt trigger signal 0"]
pub struct INT_MASK_N_0_R(crate::FieldReader<bool, INT_MASK_N_0_A>);
impl INT_MASK_N_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_MASK_N_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_MASK_N_0_A {
        match self.bits {
            false => INT_MASK_N_0_A::UNMASK,
            true => INT_MASK_N_0_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == INT_MASK_N_0_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == INT_MASK_N_0_A::MASK
    }
}
impl core::ops::Deref for INT_MASK_N_0_R {
    type Target = crate::FieldReader<bool, INT_MASK_N_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_MASK_N_0` writer - Activate or deactive interrupt trigger signal 0"]
pub struct INT_MASK_N_0_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_0_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_0_A::MASK)
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
#[doc = "Activate or deactive interrupt trigger signal 1"]
pub type INT_MASK_N_1_A = INT_MASK_N_0_A;
#[doc = "Field `INT_MASK_N_1` reader - Activate or deactive interrupt trigger signal 1"]
pub type INT_MASK_N_1_R = INT_MASK_N_0_R;
#[doc = "Field `INT_MASK_N_1` writer - Activate or deactive interrupt trigger signal 1"]
pub struct INT_MASK_N_1_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_1_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_1_A::MASK)
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
#[doc = "Activate or deactive interrupt trigger signal 2"]
pub type INT_MASK_N_2_A = INT_MASK_N_0_A;
#[doc = "Field `INT_MASK_N_2` reader - Activate or deactive interrupt trigger signal 2"]
pub type INT_MASK_N_2_R = INT_MASK_N_0_R;
#[doc = "Field `INT_MASK_N_2` writer - Activate or deactive interrupt trigger signal 2"]
pub struct INT_MASK_N_2_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_2_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_2_A::MASK)
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
#[doc = "Activate or deactive interrupt trigger signal 3"]
pub type INT_MASK_N_3_A = INT_MASK_N_0_A;
#[doc = "Field `INT_MASK_N_3` reader - Activate or deactive interrupt trigger signal 3"]
pub type INT_MASK_N_3_R = INT_MASK_N_0_R;
#[doc = "Field `INT_MASK_N_3` writer - Activate or deactive interrupt trigger signal 3"]
pub struct INT_MASK_N_3_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_3_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_3_A::MASK)
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
#[doc = "Activate or deactive interrupt trigger signal 4"]
pub type INT_MASK_N_4_A = INT_MASK_N_0_A;
#[doc = "Field `INT_MASK_N_4` reader - Activate or deactive interrupt trigger signal 4"]
pub type INT_MASK_N_4_R = INT_MASK_N_0_R;
#[doc = "Field `INT_MASK_N_4` writer - Activate or deactive interrupt trigger signal 4"]
pub struct INT_MASK_N_4_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_4_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_4_A::MASK)
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
#[doc = "Activate or deactive interrupt trigger signal 5"]
pub type INT_MASK_N_5_A = INT_MASK_N_0_A;
#[doc = "Field `INT_MASK_N_5` reader - Activate or deactive interrupt trigger signal 5"]
pub type INT_MASK_N_5_R = INT_MASK_N_0_R;
#[doc = "Field `INT_MASK_N_5` writer - Activate or deactive interrupt trigger signal 5"]
pub struct INT_MASK_N_5_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_5_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_5_A::MASK)
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
#[doc = "Activate or deactive interrupt trigger signal 6"]
pub type INT_MASK_N_6_A = INT_MASK_N_0_A;
#[doc = "Field `INT_MASK_N_6` reader - Activate or deactive interrupt trigger signal 6"]
pub type INT_MASK_N_6_R = INT_MASK_N_0_R;
#[doc = "Field `INT_MASK_N_6` writer - Activate or deactive interrupt trigger signal 6"]
pub struct INT_MASK_N_6_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_6_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_6_A::MASK)
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
#[doc = "Activate or deactive interrupt trigger signal 7"]
pub type INT_MASK_N_7_A = INT_MASK_N_0_A;
#[doc = "Field `INT_MASK_N_7` reader - Activate or deactive interrupt trigger signal 7"]
pub type INT_MASK_N_7_R = INT_MASK_N_0_R;
#[doc = "Field `INT_MASK_N_7` writer - Activate or deactive interrupt trigger signal 7"]
pub struct INT_MASK_N_7_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASK_N_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MASK_N_7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(INT_MASK_N_7_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(INT_MASK_N_7_A::MASK)
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
#[doc = "Field `FFE_TO_PERIOD` reader - FFE Kick Off (timeout) perdiod configuration (value in registers = period, max value is 100). NOTE : If 0x0, The Event Counter will be turn off and all the Signals to PMU will be de-asserted. NOTE : If PMU_TO_PERIOD is 0x0, FFE Kick off signal will not trigger until FFE is alive."]
pub struct FFE_TO_PERIOD_R(crate::FieldReader<u8, u8>);
impl FFE_TO_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFE_TO_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_TO_PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_TO_PERIOD` writer - FFE Kick Off (timeout) perdiod configuration (value in registers = period, max value is 100). NOTE : If 0x0, The Event Counter will be turn off and all the Signals to PMU will be de-asserted. NOTE : If PMU_TO_PERIOD is 0x0, FFE Kick off signal will not trigger until FFE is alive."]
pub struct FFE_TO_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_TO_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 10)) | ((value as u32 & 0xff) << 10);
        self.w
    }
}
#[doc = "Field `PMU_TO_PERIO` reader - PMU Kick Off (Time out Period) Configuration PMU Kick Off signal will assert N cycles (value in field) before FFE kick signal asserting. NOTE : For 0x0, FFE Time out Event will be used to wake up FFE power domain."]
pub struct PMU_TO_PERIO_R(crate::FieldReader<u8, u8>);
impl PMU_TO_PERIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMU_TO_PERIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMU_TO_PERIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMU_TO_PERIO` writer - PMU Kick Off (Time out Period) Configuration PMU Kick Off signal will assert N cycles (value in field) before FFE kick signal asserting. NOTE : For 0x0, FFE Time out Event will be used to wake up FFE power domain."]
pub struct PMU_TO_PERIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_TO_PERIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Field to enable the timer/counter"]
    #[inline(always)]
    pub fn spt_en(&self) -> SPT_EN_R {
        SPT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select the clock source"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Activate or deactive interrupt trigger signal 0"]
    #[inline(always)]
    pub fn int_mask_n_0(&self) -> INT_MASK_N_0_R {
        INT_MASK_N_0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Activate or deactive interrupt trigger signal 1"]
    #[inline(always)]
    pub fn int_mask_n_1(&self) -> INT_MASK_N_1_R {
        INT_MASK_N_1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Activate or deactive interrupt trigger signal 2"]
    #[inline(always)]
    pub fn int_mask_n_2(&self) -> INT_MASK_N_2_R {
        INT_MASK_N_2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Activate or deactive interrupt trigger signal 3"]
    #[inline(always)]
    pub fn int_mask_n_3(&self) -> INT_MASK_N_3_R {
        INT_MASK_N_3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Activate or deactive interrupt trigger signal 4"]
    #[inline(always)]
    pub fn int_mask_n_4(&self) -> INT_MASK_N_4_R {
        INT_MASK_N_4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Activate or deactive interrupt trigger signal 5"]
    #[inline(always)]
    pub fn int_mask_n_5(&self) -> INT_MASK_N_5_R {
        INT_MASK_N_5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Activate or deactive interrupt trigger signal 6"]
    #[inline(always)]
    pub fn int_mask_n_6(&self) -> INT_MASK_N_6_R {
        INT_MASK_N_6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Activate or deactive interrupt trigger signal 7"]
    #[inline(always)]
    pub fn int_mask_n_7(&self) -> INT_MASK_N_7_R {
        INT_MASK_N_7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:17 - FFE Kick Off (timeout) perdiod configuration (value in registers = period, max value is 100). NOTE : If 0x0, The Event Counter will be turn off and all the Signals to PMU will be de-asserted. NOTE : If PMU_TO_PERIOD is 0x0, FFE Kick off signal will not trigger until FFE is alive."]
    #[inline(always)]
    pub fn ffe_to_period(&self) -> FFE_TO_PERIOD_R {
        FFE_TO_PERIOD_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:21 - PMU Kick Off (Time out Period) Configuration PMU Kick Off signal will assert N cycles (value in field) before FFE kick signal asserting. NOTE : For 0x0, FFE Time out Event will be used to wake up FFE power domain."]
    #[inline(always)]
    pub fn pmu_to_perio(&self) -> PMU_TO_PERIO_R {
        PMU_TO_PERIO_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Field to enable the timer/counter"]
    #[inline(always)]
    pub fn spt_en(&mut self) -> SPT_EN_W {
        SPT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Select the clock source"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W {
        CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Activate or deactive interrupt trigger signal 0"]
    #[inline(always)]
    pub fn int_mask_n_0(&mut self) -> INT_MASK_N_0_W {
        INT_MASK_N_0_W { w: self }
    }
    #[doc = "Bit 3 - Activate or deactive interrupt trigger signal 1"]
    #[inline(always)]
    pub fn int_mask_n_1(&mut self) -> INT_MASK_N_1_W {
        INT_MASK_N_1_W { w: self }
    }
    #[doc = "Bit 4 - Activate or deactive interrupt trigger signal 2"]
    #[inline(always)]
    pub fn int_mask_n_2(&mut self) -> INT_MASK_N_2_W {
        INT_MASK_N_2_W { w: self }
    }
    #[doc = "Bit 5 - Activate or deactive interrupt trigger signal 3"]
    #[inline(always)]
    pub fn int_mask_n_3(&mut self) -> INT_MASK_N_3_W {
        INT_MASK_N_3_W { w: self }
    }
    #[doc = "Bit 6 - Activate or deactive interrupt trigger signal 4"]
    #[inline(always)]
    pub fn int_mask_n_4(&mut self) -> INT_MASK_N_4_W {
        INT_MASK_N_4_W { w: self }
    }
    #[doc = "Bit 7 - Activate or deactive interrupt trigger signal 5"]
    #[inline(always)]
    pub fn int_mask_n_5(&mut self) -> INT_MASK_N_5_W {
        INT_MASK_N_5_W { w: self }
    }
    #[doc = "Bit 8 - Activate or deactive interrupt trigger signal 6"]
    #[inline(always)]
    pub fn int_mask_n_6(&mut self) -> INT_MASK_N_6_W {
        INT_MASK_N_6_W { w: self }
    }
    #[doc = "Bit 9 - Activate or deactive interrupt trigger signal 7"]
    #[inline(always)]
    pub fn int_mask_n_7(&mut self) -> INT_MASK_N_7_W {
        INT_MASK_N_7_W { w: self }
    }
    #[doc = "Bits 10:17 - FFE Kick Off (timeout) perdiod configuration (value in registers = period, max value is 100). NOTE : If 0x0, The Event Counter will be turn off and all the Signals to PMU will be de-asserted. NOTE : If PMU_TO_PERIOD is 0x0, FFE Kick off signal will not trigger until FFE is alive."]
    #[inline(always)]
    pub fn ffe_to_period(&mut self) -> FFE_TO_PERIOD_W {
        FFE_TO_PERIOD_W { w: self }
    }
    #[doc = "Bits 18:21 - PMU Kick Off (Time out Period) Configuration PMU Kick Off signal will assert N cycles (value in field) before FFE kick signal asserting. NOTE : For 0x0, FFE Time out Event will be used to wake up FFE power domain."]
    #[inline(always)]
    pub fn pmu_to_perio(&mut self) -> PMU_TO_PERIO_W {
        PMU_TO_PERIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for the simple periodic timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spt_cfg](index.html) module"]
pub struct SPT_CFG_SPEC;
impl crate::RegisterSpec for SPT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spt_cfg::R](R) reader structure"]
impl crate::Readable for SPT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spt_cfg::W](W) writer structure"]
impl crate::Writable for SPT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPT_CFG to value 0"]
impl crate::Resettable for SPT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
