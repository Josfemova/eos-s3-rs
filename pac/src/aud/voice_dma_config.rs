#[doc = "Register `VOICE_DMA_CONFIG` reader"]
pub struct R(crate::R<VOICE_DMA_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOICE_DMA_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOICE_DMA_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOICE_DMA_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOICE_DMA_CONFIG` writer"]
pub struct W(crate::W<VOICE_DMA_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOICE_DMA_CONFIG_SPEC>;
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
impl From<crate::W<VOICE_DMA_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOICE_DMA_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAC_EN` reader - Set to enable Voice DMAC"]
pub struct DMAC_EN_R(crate::FieldReader<bool, bool>);
impl DMAC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC_EN` writer - Set to enable Voice DMAC"]
pub struct DMAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_EN_W<'a> {
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
#[doc = "Field `DMAC_START` writer - Set to start DMA"]
pub struct DMAC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_START_W<'a> {
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
#[doc = "Field `DMAC_STOP` reader - Set to Stop DMA"]
pub struct DMAC_STOP_R(crate::FieldReader<bool, bool>);
impl DMAC_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC_STOP` writer - Set to Stop DMA"]
pub struct DMAC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_STOP_W<'a> {
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
#[doc = "Field `AHB_RDY` reader - Set to indicate AHB clock is ready"]
pub struct AHB_RDY_R(crate::FieldReader<bool, bool>);
impl AHB_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_RDY` writer - Set to indicate AHB clock is ready"]
pub struct AHB_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_RDY_W<'a> {
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
#[doc = "Field `AHB_BURST_LENGHT` reader - Select the AHB Burst Lenght in words"]
pub struct AHB_BURST_LENGHT_R(crate::FieldReader<u8, u8>);
impl AHB_BURST_LENGHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHB_BURST_LENGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_BURST_LENGHT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_BURST_LENGHT` writer - Select the AHB Burst Lenght in words"]
pub struct AHB_BURST_LENGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_BURST_LENGHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PINGPONG_MODE` reader - Select the pingpong mode"]
pub struct PINGPONG_MODE_R(crate::FieldReader<bool, bool>);
impl PINGPONG_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PINGPONG_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINGPONG_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINGPONG_MODE` writer - Select the pingpong mode"]
pub struct PINGPONG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PINGPONG_MODE_W<'a> {
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
#[doc = "Field `STEREO_DUAL_BUF_MODE` reader - Select how to process stereo buffer data"]
pub struct STEREO_DUAL_BUF_MODE_R(crate::FieldReader<bool, bool>);
impl STEREO_DUAL_BUF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STEREO_DUAL_BUF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STEREO_DUAL_BUF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STEREO_DUAL_BUF_MODE` writer - Select how to process stereo buffer data"]
pub struct STEREO_DUAL_BUF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> STEREO_DUAL_BUF_MODE_W<'a> {
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
#[doc = "Field `VOICE_DMAC_BURST_SPD` reader - AHB cycles between two consecutive AHB Bursts"]
pub struct VOICE_DMAC_BURST_SPD_R(crate::FieldReader<u8, u8>);
impl VOICE_DMAC_BURST_SPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VOICE_DMAC_BURST_SPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOICE_DMAC_BURST_SPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOICE_DMAC_BURST_SPD` writer - AHB cycles between two consecutive AHB Bursts"]
pub struct VOICE_DMAC_BURST_SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> VOICE_DMAC_BURST_SPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set to enable Voice DMAC"]
    #[inline(always)]
    pub fn dmac_en(&self) -> DMAC_EN_R {
        DMAC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to Stop DMA"]
    #[inline(always)]
    pub fn dmac_stop(&self) -> DMAC_STOP_R {
        DMAC_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to indicate AHB clock is ready"]
    #[inline(always)]
    pub fn ahb_rdy(&self) -> AHB_RDY_R {
        AHB_RDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Select the AHB Burst Lenght in words"]
    #[inline(always)]
    pub fn ahb_burst_lenght(&self) -> AHB_BURST_LENGHT_R {
        AHB_BURST_LENGHT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Select the pingpong mode"]
    #[inline(always)]
    pub fn pingpong_mode(&self) -> PINGPONG_MODE_R {
        PINGPONG_MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Select how to process stereo buffer data"]
    #[inline(always)]
    pub fn stereo_dual_buf_mode(&self) -> STEREO_DUAL_BUF_MODE_R {
        STEREO_DUAL_BUF_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - AHB cycles between two consecutive AHB Bursts"]
    #[inline(always)]
    pub fn voice_dmac_burst_spd(&self) -> VOICE_DMAC_BURST_SPD_R {
        VOICE_DMAC_BURST_SPD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable Voice DMAC"]
    #[inline(always)]
    pub fn dmac_en(&mut self) -> DMAC_EN_W {
        DMAC_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set to start DMA"]
    #[inline(always)]
    pub fn dmac_start(&mut self) -> DMAC_START_W {
        DMAC_START_W { w: self }
    }
    #[doc = "Bit 2 - Set to Stop DMA"]
    #[inline(always)]
    pub fn dmac_stop(&mut self) -> DMAC_STOP_W {
        DMAC_STOP_W { w: self }
    }
    #[doc = "Bit 3 - Set to indicate AHB clock is ready"]
    #[inline(always)]
    pub fn ahb_rdy(&mut self) -> AHB_RDY_W {
        AHB_RDY_W { w: self }
    }
    #[doc = "Bits 4:5 - Select the AHB Burst Lenght in words"]
    #[inline(always)]
    pub fn ahb_burst_lenght(&mut self) -> AHB_BURST_LENGHT_W {
        AHB_BURST_LENGHT_W { w: self }
    }
    #[doc = "Bit 6 - Select the pingpong mode"]
    #[inline(always)]
    pub fn pingpong_mode(&mut self) -> PINGPONG_MODE_W {
        PINGPONG_MODE_W { w: self }
    }
    #[doc = "Bit 7 - Select how to process stereo buffer data"]
    #[inline(always)]
    pub fn stereo_dual_buf_mode(&mut self) -> STEREO_DUAL_BUF_MODE_W {
        STEREO_DUAL_BUF_MODE_W { w: self }
    }
    #[doc = "Bits 8:15 - AHB cycles between two consecutive AHB Bursts"]
    #[inline(always)]
    pub fn voice_dmac_burst_spd(&mut self) -> VOICE_DMAC_BURST_SPD_W {
        VOICE_DMAC_BURST_SPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio DMAC configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voice_dma_config](index.html) module"]
pub struct VOICE_DMA_CONFIG_SPEC;
impl crate::RegisterSpec for VOICE_DMA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voice_dma_config::R](R) reader structure"]
impl crate::Readable for VOICE_DMA_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voice_dma_config::W](W) writer structure"]
impl crate::Writable for VOICE_DMA_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOICE_DMA_CONFIG to value 0"]
impl crate::Resettable for VOICE_DMA_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
