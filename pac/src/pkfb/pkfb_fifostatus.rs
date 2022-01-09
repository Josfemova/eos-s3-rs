#[doc = "Register `PKFB_FIFOSTATUS` reader"]
pub struct R(crate::R<PKFB_FIFOSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_FIFOSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_FIFOSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_FIFOSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_FIFOSTATUS` writer"]
pub struct W(crate::W<PKFB_FIFOSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_FIFOSTATUS_SPEC>;
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
impl From<crate::W<PKFB_FIFOSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_FIFOSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM Sleep Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PF0_SRAM_SLEEP_A {
    #[doc = "0: SRAM is in active mode"]
    ACTIVE = 0,
    #[doc = "1: SRAM is in Light Sleep mode"]
    LIGHT_SLEEP = 1,
    #[doc = "2: SRAM is in Deep Sleep mode"]
    DEEP_SLEEP = 2,
    #[doc = "3: SRAM is in Shutdown mode"]
    SHUTDOWN = 3,
}
impl From<PF0_SRAM_SLEEP_A> for u8 {
    #[inline(always)]
    fn from(variant: PF0_SRAM_SLEEP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pf0_sram_sleep` reader - SRAM Sleep Status"]
pub struct PF0_SRAM_SLEEP_R(crate::FieldReader<u8, PF0_SRAM_SLEEP_A>);
impl PF0_SRAM_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF0_SRAM_SLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_SRAM_SLEEP_A {
        match self.bits {
            0 => PF0_SRAM_SLEEP_A::ACTIVE,
            1 => PF0_SRAM_SLEEP_A::LIGHT_SLEEP,
            2 => PF0_SRAM_SLEEP_A::DEEP_SLEEP,
            3 => PF0_SRAM_SLEEP_A::SHUTDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == PF0_SRAM_SLEEP_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        **self == PF0_SRAM_SLEEP_A::LIGHT_SLEEP
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        **self == PF0_SRAM_SLEEP_A::DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == PF0_SRAM_SLEEP_A::SHUTDOWN
    }
}
impl core::ops::Deref for PF0_SRAM_SLEEP_R {
    type Target = crate::FieldReader<u8, PF0_SRAM_SLEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_sram_sleep` writer - SRAM Sleep Status"]
pub struct PF0_SRAM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_SRAM_SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_SRAM_SLEEP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SRAM is in active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(PF0_SRAM_SLEEP_A::ACTIVE)
    }
    #[doc = "SRAM is in Light Sleep mode"]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(PF0_SRAM_SLEEP_A::LIGHT_SLEEP)
    }
    #[doc = "SRAM is in Deep Sleep mode"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(PF0_SRAM_SLEEP_A::DEEP_SLEEP)
    }
    #[doc = "SRAM is in Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(PF0_SRAM_SLEEP_A::SHUTDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `pf0_push_int_over` reader - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF0_PUSH_INT_OVER_R(crate::FieldReader<bool, bool>);
impl PF0_PUSH_INT_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_PUSH_INT_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_PUSH_INT_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_push_int_over` writer - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF0_PUSH_INT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_PUSH_INT_OVER_W<'a> {
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
#[doc = "Field `pf0_push_int_thresh` reader - Bit is set if there's an interrupt set for push threshold"]
pub struct PF0_PUSH_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF0_PUSH_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_PUSH_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_PUSH_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_push_int_thresh` writer - Bit is set if there's an interrupt set for push threshold"]
pub struct PF0_PUSH_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_PUSH_INT_THRESH_W<'a> {
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
#[doc = "Field `pf0_push_int_sleep` reader - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF0_PUSH_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF0_PUSH_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_PUSH_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_PUSH_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_push_int_sleep` writer - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF0_PUSH_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_PUSH_INT_SLEEP_W<'a> {
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
#[doc = "Field `pf0_pop_int_under` reader - Bit is set if there's an interrupt set for underflow"]
pub struct PF0_POP_INT_UNDER_R(crate::FieldReader<bool, bool>);
impl PF0_POP_INT_UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_INT_UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_POP_INT_UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_int_under` writer - Bit is set if there's an interrupt set for underflow"]
pub struct PF0_POP_INT_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_UNDER_W<'a> {
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
#[doc = "Field `pf0_pop_int_thresh` reader - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF0_POP_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF0_POP_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_POP_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_int_thresh` writer - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF0_POP_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_THRESH_W<'a> {
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
#[doc = "Field `pf0_pop_int_sleep` reader - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF0_POP_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF0_POP_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_POP_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_int_sleep` writer - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF0_POP_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_SLEEP_W<'a> {
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
#[doc = "SRAM Sleep Status"]
pub type PF1_SRAM_SLEEP_A = PF0_SRAM_SLEEP_A;
#[doc = "Field `pf1_sram_sleep` reader - SRAM Sleep Status"]
pub type PF1_SRAM_SLEEP_R = PF0_SRAM_SLEEP_R;
#[doc = "Field `pf1_sram_sleep` writer - SRAM Sleep Status"]
pub struct PF1_SRAM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_SRAM_SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_SRAM_SLEEP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SRAM is in active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(PF1_SRAM_SLEEP_A::ACTIVE)
    }
    #[doc = "SRAM is in Light Sleep mode"]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(PF1_SRAM_SLEEP_A::LIGHT_SLEEP)
    }
    #[doc = "SRAM is in Deep Sleep mode"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(PF1_SRAM_SLEEP_A::DEEP_SLEEP)
    }
    #[doc = "SRAM is in Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(PF1_SRAM_SLEEP_A::SHUTDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `pf1_push_int_over` reader - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF1_PUSH_INT_OVER_R(crate::FieldReader<bool, bool>);
impl PF1_PUSH_INT_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_PUSH_INT_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_PUSH_INT_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_push_int_over` writer - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF1_PUSH_INT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_PUSH_INT_OVER_W<'a> {
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
#[doc = "Field `pf1_push_int_thresh` reader - Bit is set if there's an interrupt set for push threshold"]
pub struct PF1_PUSH_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF1_PUSH_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_PUSH_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_PUSH_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_push_int_thresh` writer - Bit is set if there's an interrupt set for push threshold"]
pub struct PF1_PUSH_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_PUSH_INT_THRESH_W<'a> {
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
#[doc = "Field `pf1_push_int_sleep` reader - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF1_PUSH_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF1_PUSH_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_PUSH_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_PUSH_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_push_int_sleep` writer - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF1_PUSH_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_PUSH_INT_SLEEP_W<'a> {
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
#[doc = "Field `pf1_pop_int_under` reader - Bit is set if there's an interrupt set for underflow"]
pub struct PF1_POP_INT_UNDER_R(crate::FieldReader<bool, bool>);
impl PF1_POP_INT_UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_POP_INT_UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_POP_INT_UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_pop_int_under` writer - Bit is set if there's an interrupt set for underflow"]
pub struct PF1_POP_INT_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_POP_INT_UNDER_W<'a> {
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
#[doc = "Field `pf1_pop_int_thresh` reader - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF1_POP_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF1_POP_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_POP_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_POP_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_pop_int_thresh` writer - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF1_POP_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_POP_INT_THRESH_W<'a> {
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
#[doc = "Field `pf1_pop_int_sleep` reader - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF1_POP_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF1_POP_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF1_POP_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF1_POP_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf1_pop_int_sleep` writer - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF1_POP_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_POP_INT_SLEEP_W<'a> {
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
#[doc = "SRAM Sleep Status"]
pub type PF2_SRAM_SLEEP_A = PF0_SRAM_SLEEP_A;
#[doc = "Field `pf2_sram_sleep` reader - SRAM Sleep Status"]
pub type PF2_SRAM_SLEEP_R = PF0_SRAM_SLEEP_R;
#[doc = "Field `pf2_sram_sleep` writer - SRAM Sleep Status"]
pub struct PF2_SRAM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_SRAM_SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_SRAM_SLEEP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SRAM is in active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(PF2_SRAM_SLEEP_A::ACTIVE)
    }
    #[doc = "SRAM is in Light Sleep mode"]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(PF2_SRAM_SLEEP_A::LIGHT_SLEEP)
    }
    #[doc = "SRAM is in Deep Sleep mode"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(PF2_SRAM_SLEEP_A::DEEP_SLEEP)
    }
    #[doc = "SRAM is in Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(PF2_SRAM_SLEEP_A::SHUTDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `pf2_push_int_over` reader - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF2_PUSH_INT_OVER_R(crate::FieldReader<bool, bool>);
impl PF2_PUSH_INT_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_PUSH_INT_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_PUSH_INT_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_push_int_over` writer - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF2_PUSH_INT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_PUSH_INT_OVER_W<'a> {
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
#[doc = "Field `pf2_push_int_thresh` reader - Bit is set if there's an interrupt set for push threshold"]
pub struct PF2_PUSH_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF2_PUSH_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_PUSH_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_PUSH_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_push_int_thresh` writer - Bit is set if there's an interrupt set for push threshold"]
pub struct PF2_PUSH_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_PUSH_INT_THRESH_W<'a> {
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
#[doc = "Field `pf2_push_int_sleep` reader - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF2_PUSH_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF2_PUSH_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_PUSH_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_PUSH_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_push_int_sleep` writer - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF2_PUSH_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_PUSH_INT_SLEEP_W<'a> {
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
#[doc = "Field `pf2_pop_int_under` reader - Bit is set if there's an interrupt set for underflow"]
pub struct PF2_POP_INT_UNDER_R(crate::FieldReader<bool, bool>);
impl PF2_POP_INT_UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_POP_INT_UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_POP_INT_UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_pop_int_under` writer - Bit is set if there's an interrupt set for underflow"]
pub struct PF2_POP_INT_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_POP_INT_UNDER_W<'a> {
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
#[doc = "Field `pf2_pop_int_thresh` reader - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF2_POP_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF2_POP_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_POP_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_POP_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_pop_int_thresh` writer - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF2_POP_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_POP_INT_THRESH_W<'a> {
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
#[doc = "Field `pf2_pop_int_sleep` reader - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF2_POP_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF2_POP_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_POP_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_POP_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_pop_int_sleep` writer - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF2_POP_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_POP_INT_SLEEP_W<'a> {
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
            (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "SRAM Sleep Status"]
pub type PF8K_SRAM_SLEEP_A = PF0_SRAM_SLEEP_A;
#[doc = "Field `pf8k_sram_sleep` reader - SRAM Sleep Status"]
pub type PF8K_SRAM_SLEEP_R = PF0_SRAM_SLEEP_R;
#[doc = "Field `pf8k_sram_sleep` writer - SRAM Sleep Status"]
pub struct PF8K_SRAM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_SRAM_SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_SRAM_SLEEP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SRAM is in active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(PF8K_SRAM_SLEEP_A::ACTIVE)
    }
    #[doc = "SRAM is in Light Sleep mode"]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(PF8K_SRAM_SLEEP_A::LIGHT_SLEEP)
    }
    #[doc = "SRAM is in Deep Sleep mode"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(PF8K_SRAM_SLEEP_A::DEEP_SLEEP)
    }
    #[doc = "SRAM is in Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(PF8K_SRAM_SLEEP_A::SHUTDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `pf8k_push_int_over` reader - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF8K_PUSH_INT_OVER_R(crate::FieldReader<bool, bool>);
impl PF8K_PUSH_INT_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_PUSH_INT_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_PUSH_INT_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_push_int_over` writer - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
pub struct PF8K_PUSH_INT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_PUSH_INT_OVER_W<'a> {
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
            (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `pf8k_push_int_thresh` reader - Bit is set if there's an interrupt set for push threshold"]
pub struct PF8K_PUSH_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF8K_PUSH_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_PUSH_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_PUSH_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_push_int_thresh` writer - Bit is set if there's an interrupt set for push threshold"]
pub struct PF8K_PUSH_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_PUSH_INT_THRESH_W<'a> {
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
            (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `pf8k_push_int_sleep` reader - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF8K_PUSH_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF8K_PUSH_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_PUSH_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_PUSH_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_push_int_sleep` writer - Bit is set if there's an interrupt set for push on SRAM sleep"]
pub struct PF8K_PUSH_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_PUSH_INT_SLEEP_W<'a> {
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
            (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `pf8k_pop_int_under` reader - Bit is set if there's an interrupt set for underflow"]
pub struct PF8K_POP_INT_UNDER_R(crate::FieldReader<bool, bool>);
impl PF8K_POP_INT_UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_POP_INT_UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_POP_INT_UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_pop_int_under` writer - Bit is set if there's an interrupt set for underflow"]
pub struct PF8K_POP_INT_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_POP_INT_UNDER_W<'a> {
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
            (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `pf8k_pop_int_thresh` reader - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF8K_POP_INT_THRESH_R(crate::FieldReader<bool, bool>);
impl PF8K_POP_INT_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_POP_INT_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_POP_INT_THRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_pop_int_thresh` writer - Bit is set if there's an interrupt set for pop threshold"]
pub struct PF8K_POP_INT_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_POP_INT_THRESH_W<'a> {
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
            (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `pf8k_pop_int_sleep` reader - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF8K_POP_INT_SLEEP_R(crate::FieldReader<bool, bool>);
impl PF8K_POP_INT_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_POP_INT_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_POP_INT_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_pop_int_sleep` writer - Bit is set if there's an interrupt set for pop on SRAM sleep"]
pub struct PF8K_POP_INT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_POP_INT_SLEEP_W<'a> {
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
            (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf0_sram_sleep(&self) -> PF0_SRAM_SLEEP_R {
        PF0_SRAM_SLEEP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf0_push_int_over(&self) -> PF0_PUSH_INT_OVER_R {
        PF0_PUSH_INT_OVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf0_push_int_thresh(&self) -> PF0_PUSH_INT_THRESH_R {
        PF0_PUSH_INT_THRESH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf0_push_int_sleep(&self) -> PF0_PUSH_INT_SLEEP_R {
        PF0_PUSH_INT_SLEEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf0_pop_int_under(&self) -> PF0_POP_INT_UNDER_R {
        PF0_POP_INT_UNDER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf0_pop_int_thresh(&self) -> PF0_POP_INT_THRESH_R {
        PF0_POP_INT_THRESH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf0_pop_int_sleep(&self) -> PF0_POP_INT_SLEEP_R {
        PF0_POP_INT_SLEEP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf1_sram_sleep(&self) -> PF1_SRAM_SLEEP_R {
        PF1_SRAM_SLEEP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf1_push_int_over(&self) -> PF1_PUSH_INT_OVER_R {
        PF1_PUSH_INT_OVER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf1_push_int_thresh(&self) -> PF1_PUSH_INT_THRESH_R {
        PF1_PUSH_INT_THRESH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf1_push_int_sleep(&self) -> PF1_PUSH_INT_SLEEP_R {
        PF1_PUSH_INT_SLEEP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf1_pop_int_under(&self) -> PF1_POP_INT_UNDER_R {
        PF1_POP_INT_UNDER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf1_pop_int_thresh(&self) -> PF1_POP_INT_THRESH_R {
        PF1_POP_INT_THRESH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf1_pop_int_sleep(&self) -> PF1_POP_INT_SLEEP_R {
        PF1_POP_INT_SLEEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf2_sram_sleep(&self) -> PF2_SRAM_SLEEP_R {
        PF2_SRAM_SLEEP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf2_push_int_over(&self) -> PF2_PUSH_INT_OVER_R {
        PF2_PUSH_INT_OVER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf2_push_int_thresh(&self) -> PF2_PUSH_INT_THRESH_R {
        PF2_PUSH_INT_THRESH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf2_push_int_sleep(&self) -> PF2_PUSH_INT_SLEEP_R {
        PF2_PUSH_INT_SLEEP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf2_pop_int_under(&self) -> PF2_POP_INT_UNDER_R {
        PF2_POP_INT_UNDER_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf2_pop_int_thresh(&self) -> PF2_POP_INT_THRESH_R {
        PF2_POP_INT_THRESH_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf2_pop_int_sleep(&self) -> PF2_POP_INT_SLEEP_R {
        PF2_POP_INT_SLEEP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf8k_sram_sleep(&self) -> PF8K_SRAM_SLEEP_R {
        PF8K_SRAM_SLEEP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf8k_push_int_over(&self) -> PF8K_PUSH_INT_OVER_R {
        PF8K_PUSH_INT_OVER_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf8k_push_int_thresh(&self) -> PF8K_PUSH_INT_THRESH_R {
        PF8K_PUSH_INT_THRESH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf8k_push_int_sleep(&self) -> PF8K_PUSH_INT_SLEEP_R {
        PF8K_PUSH_INT_SLEEP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf8k_pop_int_under(&self) -> PF8K_POP_INT_UNDER_R {
        PF8K_POP_INT_UNDER_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf8k_pop_int_thresh(&self) -> PF8K_POP_INT_THRESH_R {
        PF8K_POP_INT_THRESH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf8k_pop_int_sleep(&self) -> PF8K_POP_INT_SLEEP_R {
        PF8K_POP_INT_SLEEP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf0_sram_sleep(&mut self) -> PF0_SRAM_SLEEP_W {
        PF0_SRAM_SLEEP_W { w: self }
    }
    #[doc = "Bit 2 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf0_push_int_over(&mut self) -> PF0_PUSH_INT_OVER_W {
        PF0_PUSH_INT_OVER_W { w: self }
    }
    #[doc = "Bit 3 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf0_push_int_thresh(&mut self) -> PF0_PUSH_INT_THRESH_W {
        PF0_PUSH_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 4 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf0_push_int_sleep(&mut self) -> PF0_PUSH_INT_SLEEP_W {
        PF0_PUSH_INT_SLEEP_W { w: self }
    }
    #[doc = "Bit 5 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf0_pop_int_under(&mut self) -> PF0_POP_INT_UNDER_W {
        PF0_POP_INT_UNDER_W { w: self }
    }
    #[doc = "Bit 6 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf0_pop_int_thresh(&mut self) -> PF0_POP_INT_THRESH_W {
        PF0_POP_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 7 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf0_pop_int_sleep(&mut self) -> PF0_POP_INT_SLEEP_W {
        PF0_POP_INT_SLEEP_W { w: self }
    }
    #[doc = "Bits 8:9 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf1_sram_sleep(&mut self) -> PF1_SRAM_SLEEP_W {
        PF1_SRAM_SLEEP_W { w: self }
    }
    #[doc = "Bit 10 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf1_push_int_over(&mut self) -> PF1_PUSH_INT_OVER_W {
        PF1_PUSH_INT_OVER_W { w: self }
    }
    #[doc = "Bit 11 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf1_push_int_thresh(&mut self) -> PF1_PUSH_INT_THRESH_W {
        PF1_PUSH_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 12 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf1_push_int_sleep(&mut self) -> PF1_PUSH_INT_SLEEP_W {
        PF1_PUSH_INT_SLEEP_W { w: self }
    }
    #[doc = "Bit 13 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf1_pop_int_under(&mut self) -> PF1_POP_INT_UNDER_W {
        PF1_POP_INT_UNDER_W { w: self }
    }
    #[doc = "Bit 14 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf1_pop_int_thresh(&mut self) -> PF1_POP_INT_THRESH_W {
        PF1_POP_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 15 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf1_pop_int_sleep(&mut self) -> PF1_POP_INT_SLEEP_W {
        PF1_POP_INT_SLEEP_W { w: self }
    }
    #[doc = "Bits 16:17 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf2_sram_sleep(&mut self) -> PF2_SRAM_SLEEP_W {
        PF2_SRAM_SLEEP_W { w: self }
    }
    #[doc = "Bit 18 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf2_push_int_over(&mut self) -> PF2_PUSH_INT_OVER_W {
        PF2_PUSH_INT_OVER_W { w: self }
    }
    #[doc = "Bit 19 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf2_push_int_thresh(&mut self) -> PF2_PUSH_INT_THRESH_W {
        PF2_PUSH_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 20 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf2_push_int_sleep(&mut self) -> PF2_PUSH_INT_SLEEP_W {
        PF2_PUSH_INT_SLEEP_W { w: self }
    }
    #[doc = "Bit 21 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf2_pop_int_under(&mut self) -> PF2_POP_INT_UNDER_W {
        PF2_POP_INT_UNDER_W { w: self }
    }
    #[doc = "Bit 22 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf2_pop_int_thresh(&mut self) -> PF2_POP_INT_THRESH_W {
        PF2_POP_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 23 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf2_pop_int_sleep(&mut self) -> PF2_POP_INT_SLEEP_W {
        PF2_POP_INT_SLEEP_W { w: self }
    }
    #[doc = "Bits 24:25 - SRAM Sleep Status"]
    #[inline(always)]
    pub fn pf8k_sram_sleep(&mut self) -> PF8K_SRAM_SLEEP_W {
        PF8K_SRAM_SLEEP_W { w: self }
    }
    #[doc = "Bit 26 - Bit is set if there's an interrupt set for overflow (pktFIFO or FFE FIFO)"]
    #[inline(always)]
    pub fn pf8k_push_int_over(&mut self) -> PF8K_PUSH_INT_OVER_W {
        PF8K_PUSH_INT_OVER_W { w: self }
    }
    #[doc = "Bit 27 - Bit is set if there's an interrupt set for push threshold"]
    #[inline(always)]
    pub fn pf8k_push_int_thresh(&mut self) -> PF8K_PUSH_INT_THRESH_W {
        PF8K_PUSH_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 28 - Bit is set if there's an interrupt set for push on SRAM sleep"]
    #[inline(always)]
    pub fn pf8k_push_int_sleep(&mut self) -> PF8K_PUSH_INT_SLEEP_W {
        PF8K_PUSH_INT_SLEEP_W { w: self }
    }
    #[doc = "Bit 29 - Bit is set if there's an interrupt set for underflow"]
    #[inline(always)]
    pub fn pf8k_pop_int_under(&mut self) -> PF8K_POP_INT_UNDER_W {
        PF8K_POP_INT_UNDER_W { w: self }
    }
    #[doc = "Bit 30 - Bit is set if there's an interrupt set for pop threshold"]
    #[inline(always)]
    pub fn pf8k_pop_int_thresh(&mut self) -> PF8K_POP_INT_THRESH_W {
        PF8K_POP_INT_THRESH_W { w: self }
    }
    #[doc = "Bit 31 - Bit is set if there's an interrupt set for pop on SRAM sleep"]
    #[inline(always)]
    pub fn pf8k_pop_int_sleep(&mut self) -> PF8K_POP_INT_SLEEP_W {
        PF8K_POP_INT_SLEEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet FIFO Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_fifostatus](index.html) module"]
pub struct PKFB_FIFOSTATUS_SPEC;
impl crate::RegisterSpec for PKFB_FIFOSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_fifostatus::R](R) reader structure"]
impl crate::Readable for PKFB_FIFOSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_fifostatus::W](W) writer structure"]
impl crate::Writable for PKFB_FIFOSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_FIFOSTATUS to value 0"]
impl crate::Resettable for PKFB_FIFOSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
