#[doc = "Register `VOICE_STATUS` reader"]
pub struct R(crate::R<VOICE_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOICE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOICE_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOICE_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOICE_STATUS` writer"]
pub struct W(crate::W<VOICE_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOICE_STATUS_SPEC>;
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
impl From<crate::W<VOICE_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOICE_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_0A_EMPTY` reader - Set if the FIFO is empty"]
pub struct FIFO_0A_EMPTY_R(crate::FieldReader<bool, bool>);
impl FIFO_0A_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_0A_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_0A_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_0A_FULL` reader - Set if the FIFO is full"]
pub struct FIFO_0A_FULL_R(crate::FieldReader<bool, bool>);
impl FIFO_0A_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_0A_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_0A_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_0A_FULL` writer - Set if the FIFO is full"]
pub struct FIFO_0A_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_0A_FULL_W<'a> {
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
#[doc = "Field `FIFO_0A_OVERFLOW` reader - Set if there's a FIFO overflow"]
pub struct FIFO_0A_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl FIFO_0A_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_0A_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_0A_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_0A_OVERFLOW` writer - Set if there's a FIFO overflow"]
pub struct FIFO_0A_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_0A_OVERFLOW_W<'a> {
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
#[doc = "Field `FIFO_0B_EMPTY` reader - Set if the FIFO is empty"]
pub struct FIFO_0B_EMPTY_R(crate::FieldReader<bool, bool>);
impl FIFO_0B_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_0B_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_0B_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_0B_FULL` reader - Set if the FIFO is full"]
pub struct FIFO_0B_FULL_R(crate::FieldReader<bool, bool>);
impl FIFO_0B_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_0B_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_0B_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_0B_FULL` writer - Set if the FIFO is full"]
pub struct FIFO_0B_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_0B_FULL_W<'a> {
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
#[doc = "Field `FIFO_0B_OVERFLOW` reader - Set if there's a FIFO overflow"]
pub struct FIFO_0B_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl FIFO_0B_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_0B_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_0B_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_0B_OVERFLOW` writer - Set if there's a FIFO overflow"]
pub struct FIFO_0B_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_0B_OVERFLOW_W<'a> {
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
#[doc = "Field `FIFO_1A_EMPTY` reader - Set if the FIFO is empty"]
pub struct FIFO_1A_EMPTY_R(crate::FieldReader<bool, bool>);
impl FIFO_1A_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_1A_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_1A_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_1A_FULL` reader - Set if the FIFO is full"]
pub struct FIFO_1A_FULL_R(crate::FieldReader<bool, bool>);
impl FIFO_1A_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_1A_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_1A_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_1A_FULL` writer - Set if the FIFO is full"]
pub struct FIFO_1A_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_1A_FULL_W<'a> {
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
#[doc = "Field `FIFO_1A_OVERFLOW` reader - Set if there's a FIFO overflow"]
pub struct FIFO_1A_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl FIFO_1A_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_1A_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_1A_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_1A_OVERFLOW` writer - Set if there's a FIFO overflow"]
pub struct FIFO_1A_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_1A_OVERFLOW_W<'a> {
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
#[doc = "Field `FIFO_1B_EMPTY` reader - Set if the FIFO is empty"]
pub struct FIFO_1B_EMPTY_R(crate::FieldReader<bool, bool>);
impl FIFO_1B_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_1B_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_1B_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_1B_FULL` reader - Set if the FIFO is full"]
pub struct FIFO_1B_FULL_R(crate::FieldReader<bool, bool>);
impl FIFO_1B_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_1B_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_1B_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_1B_FULL` writer - Set if the FIFO is full"]
pub struct FIFO_1B_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_1B_FULL_W<'a> {
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
#[doc = "Field `FIFO_1B_OVERFLOW` reader - Set if there's a FIFO overflow"]
pub struct FIFO_1B_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl FIFO_1B_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_1B_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_1B_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_1B_OVERFLOW` writer - Set if there's a FIFO overflow"]
pub struct FIFO_1B_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_1B_OVERFLOW_W<'a> {
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
#[doc = "Field `DMIC_VOICE_DETECTED_REG` reader - Set if detected"]
pub struct DMIC_VOICE_DETECTED_REG_R(crate::FieldReader<bool, bool>);
impl DMIC_VOICE_DETECTED_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMIC_VOICE_DETECTED_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMIC_VOICE_DETECTED_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_VOICE_DETECTED_REG` reader - Set if detected"]
pub struct LPSD_VOICE_DETECTED_REG_R(crate::FieldReader<bool, bool>);
impl LPSD_VOICE_DETECTED_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSD_VOICE_DETECTED_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSD_VOICE_DETECTED_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_PDM_CLK_OFF_REG` reader - Set if AP PDM clock is off"]
pub struct AP_PDM_CLK_OFF_REG_R(crate::FieldReader<bool, bool>);
impl AP_PDM_CLK_OFF_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AP_PDM_CLK_OFF_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP_PDM_CLK_OFF_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_PDM_CLK_ON_REG` reader - Set if AP PDM clock is on"]
pub struct AP_PDM_CLK_ON_REG_R(crate::FieldReader<bool, bool>);
impl AP_PDM_CLK_ON_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AP_PDM_CLK_ON_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP_PDM_CLK_ON_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC1_BUF_DONE_REG` reader - Set if done"]
pub struct DMAC1_BUF_DONE_REG_R(crate::FieldReader<bool, bool>);
impl DMAC1_BUF_DONE_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC1_BUF_DONE_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC1_BUF_DONE_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC1_BUF_DONE_REG` writer - Set if done"]
pub struct DMAC1_BUF_DONE_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC1_BUF_DONE_REG_W<'a> {
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
#[doc = "Field `DMAC1_BLK_DONE_REG` reader - Set if done"]
pub struct DMAC1_BLK_DONE_REG_R(crate::FieldReader<bool, bool>);
impl DMAC1_BLK_DONE_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC1_BLK_DONE_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC1_BLK_DONE_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC1_BLK_DONE_REG` writer - Set if done"]
pub struct DMAC1_BLK_DONE_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC1_BLK_DONE_REG_W<'a> {
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
#[doc = "Field `DMAC0_BUF_DONE_REG` reader - Set if done"]
pub struct DMAC0_BUF_DONE_REG_R(crate::FieldReader<bool, bool>);
impl DMAC0_BUF_DONE_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC0_BUF_DONE_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC0_BUF_DONE_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC0_BUF_DONE_REG` writer - Set if done"]
pub struct DMAC0_BUF_DONE_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC0_BUF_DONE_REG_W<'a> {
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
#[doc = "Field `DMAC0_BLK_DONE_REG` reader - Set if done"]
pub struct DMAC0_BLK_DONE_REG_R(crate::FieldReader<bool, bool>);
impl DMAC0_BLK_DONE_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC0_BLK_DONE_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC0_BLK_DONE_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC0_BLK_DONE_REG` writer - Set if done"]
pub struct DMAC0_BLK_DONE_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC0_BLK_DONE_REG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set if the FIFO is empty"]
    #[inline(always)]
    pub fn fifo_0a_empty(&self) -> FIFO_0A_EMPTY_R {
        FIFO_0A_EMPTY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_0a_full(&self) -> FIFO_0A_FULL_R {
        FIFO_0A_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_0a_overflow(&self) -> FIFO_0A_OVERFLOW_R {
        FIFO_0A_OVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set if the FIFO is empty"]
    #[inline(always)]
    pub fn fifo_0b_empty(&self) -> FIFO_0B_EMPTY_R {
        FIFO_0B_EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_0b_full(&self) -> FIFO_0B_FULL_R {
        FIFO_0B_FULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_0b_overflow(&self) -> FIFO_0B_OVERFLOW_R {
        FIFO_0B_OVERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set if the FIFO is empty"]
    #[inline(always)]
    pub fn fifo_1a_empty(&self) -> FIFO_1A_EMPTY_R {
        FIFO_1A_EMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_1a_full(&self) -> FIFO_1A_FULL_R {
        FIFO_1A_FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_1a_overflow(&self) -> FIFO_1A_OVERFLOW_R {
        FIFO_1A_OVERFLOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set if the FIFO is empty"]
    #[inline(always)]
    pub fn fifo_1b_empty(&self) -> FIFO_1B_EMPTY_R {
        FIFO_1B_EMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_1b_full(&self) -> FIFO_1B_FULL_R {
        FIFO_1B_FULL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_1b_overflow(&self) -> FIFO_1B_OVERFLOW_R {
        FIFO_1B_OVERFLOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set if detected"]
    #[inline(always)]
    pub fn dmic_voice_detected_reg(&self) -> DMIC_VOICE_DETECTED_REG_R {
        DMIC_VOICE_DETECTED_REG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set if detected"]
    #[inline(always)]
    pub fn lpsd_voice_detected_reg(&self) -> LPSD_VOICE_DETECTED_REG_R {
        LPSD_VOICE_DETECTED_REG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set if AP PDM clock is off"]
    #[inline(always)]
    pub fn ap_pdm_clk_off_reg(&self) -> AP_PDM_CLK_OFF_REG_R {
        AP_PDM_CLK_OFF_REG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set if AP PDM clock is on"]
    #[inline(always)]
    pub fn ap_pdm_clk_on_reg(&self) -> AP_PDM_CLK_ON_REG_R {
        AP_PDM_CLK_ON_REG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set if done"]
    #[inline(always)]
    pub fn dmac1_buf_done_reg(&self) -> DMAC1_BUF_DONE_REG_R {
        DMAC1_BUF_DONE_REG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set if done"]
    #[inline(always)]
    pub fn dmac1_blk_done_reg(&self) -> DMAC1_BLK_DONE_REG_R {
        DMAC1_BLK_DONE_REG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Set if done"]
    #[inline(always)]
    pub fn dmac0_buf_done_reg(&self) -> DMAC0_BUF_DONE_REG_R {
        DMAC0_BUF_DONE_REG_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Set if done"]
    #[inline(always)]
    pub fn dmac0_blk_done_reg(&self) -> DMAC0_BLK_DONE_REG_R {
        DMAC0_BLK_DONE_REG_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_0a_full(&mut self) -> FIFO_0A_FULL_W {
        FIFO_0A_FULL_W { w: self }
    }
    #[doc = "Bit 2 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_0a_overflow(&mut self) -> FIFO_0A_OVERFLOW_W {
        FIFO_0A_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 5 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_0b_full(&mut self) -> FIFO_0B_FULL_W {
        FIFO_0B_FULL_W { w: self }
    }
    #[doc = "Bit 6 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_0b_overflow(&mut self) -> FIFO_0B_OVERFLOW_W {
        FIFO_0B_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 9 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_1a_full(&mut self) -> FIFO_1A_FULL_W {
        FIFO_1A_FULL_W { w: self }
    }
    #[doc = "Bit 10 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_1a_overflow(&mut self) -> FIFO_1A_OVERFLOW_W {
        FIFO_1A_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 13 - Set if the FIFO is full"]
    #[inline(always)]
    pub fn fifo_1b_full(&mut self) -> FIFO_1B_FULL_W {
        FIFO_1B_FULL_W { w: self }
    }
    #[doc = "Bit 14 - Set if there's a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_1b_overflow(&mut self) -> FIFO_1B_OVERFLOW_W {
        FIFO_1B_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 20 - Set if done"]
    #[inline(always)]
    pub fn dmac1_buf_done_reg(&mut self) -> DMAC1_BUF_DONE_REG_W {
        DMAC1_BUF_DONE_REG_W { w: self }
    }
    #[doc = "Bit 21 - Set if done"]
    #[inline(always)]
    pub fn dmac1_blk_done_reg(&mut self) -> DMAC1_BLK_DONE_REG_W {
        DMAC1_BLK_DONE_REG_W { w: self }
    }
    #[doc = "Bit 22 - Set if done"]
    #[inline(always)]
    pub fn dmac0_buf_done_reg(&mut self) -> DMAC0_BUF_DONE_REG_W {
        DMAC0_BUF_DONE_REG_W { w: self }
    }
    #[doc = "Bit 23 - Set if done"]
    #[inline(always)]
    pub fn dmac0_blk_done_reg(&mut self) -> DMAC0_BLK_DONE_REG_W {
        DMAC0_BLK_DONE_REG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voice_status](index.html) module"]
pub struct VOICE_STATUS_SPEC;
impl crate::RegisterSpec for VOICE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voice_status::R](R) reader structure"]
impl crate::Readable for VOICE_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voice_status::W](W) writer structure"]
impl crate::Writable for VOICE_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOICE_STATUS to value 0x0004_1111"]
impl crate::Resettable for VOICE_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_1111
    }
}
