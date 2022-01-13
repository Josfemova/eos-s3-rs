#[doc = "Register `ALT_CHN_CFG_CH14` reader"]
pub struct R(crate::R<ALT_CHN_CFG_CH14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALT_CHN_CFG_CH14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALT_CHN_CFG_CH14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALT_CHN_CFG_CH14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALT_CHN_CFG_CH14` writer"]
pub struct W(crate::W<ALT_CHN_CFG_CH14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALT_CHN_CFG_CH14_SPEC>;
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
impl From<crate::W<ALT_CHN_CFG_CH14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALT_CHN_CFG_CH14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cycle_ctrl` reader - The operating mode of the DMA cycle"]
pub struct CYCLE_CTRL_R(crate::FieldReader<u8, u8>);
impl CYCLE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CYCLE_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCLE_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cycle_ctrl` writer - The operating mode of the DMA cycle"]
pub struct CYCLE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLE_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `next_useburst` reader - Controls if the chnl_useburst_set \\[C\\]
bit is set to a 1, when the controller is performing a peripheral scatter-gather and is completing a DMA cycle that uses the alternate data structure"]
pub struct NEXT_USEBURST_R(crate::FieldReader<bool, bool>);
impl NEXT_USEBURST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NEXT_USEBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEXT_USEBURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `next_useburst` writer - Controls if the chnl_useburst_set \\[C\\]
bit is set to a 1, when the controller is performing a peripheral scatter-gather and is completing a DMA cycle that uses the alternate data structure"]
pub struct NEXT_USEBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_USEBURST_W<'a> {
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
#[doc = "Field `n_minus_1` reader - Prior to the DMA cycle commencing, these bits represent the total number of DMA transfers that the DMA cycle contains. You must set these bits according to the size of DMA cycle that you require."]
pub struct N_MINUS_1_R(crate::FieldReader<u16, u16>);
impl N_MINUS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        N_MINUS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_MINUS_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `n_minus_1` writer - Prior to the DMA cycle commencing, these bits represent the total number of DMA transfers that the DMA cycle contains. You must set these bits according to the size of DMA cycle that you require."]
pub struct N_MINUS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> N_MINUS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03ff << 4)) | ((value as u32 & 0x03ff) << 4);
        self.w
    }
}
#[doc = "Field `R_power` reader - Set these bits to control how many DMA transfers can occur before the controller rearbitrates"]
pub struct R_POWER_R(crate::FieldReader<u8, u8>);
impl R_POWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_POWER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_power` writer - Set these bits to control how many DMA transfers can occur before the controller rearbitrates"]
pub struct R_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> R_POWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `src_prot_ctrl` reader - Set the bits to control the state of HPROT\\[3:1\\]
when the controller reads the source data"]
pub struct SRC_PROT_CTRL_R(crate::FieldReader<u8, u8>);
impl SRC_PROT_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_PROT_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_PROT_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `src_prot_ctrl` writer - Set the bits to control the state of HPROT\\[3:1\\]
when the controller reads the source data"]
pub struct SRC_PROT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_PROT_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `dst_prot_ctrl` reader - Set the bits to control the state of HPROT\\[3:1\\]
when the controller writes the destination data"]
pub struct DST_PROT_CTRL_R(crate::FieldReader<u8, u8>);
impl DST_PROT_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DST_PROT_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_PROT_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dst_prot_ctrl` writer - Set the bits to control the state of HPROT\\[3:1\\]
when the controller writes the destination data"]
pub struct DST_PROT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_PROT_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `src_size` reader - Source data size"]
pub struct SRC_SIZE_R(crate::FieldReader<u8, u8>);
impl SRC_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `src_size` writer - Source data size"]
pub struct SRC_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `src_inc` reader - Source address increment"]
pub struct SRC_INC_R(crate::FieldReader<u8, u8>);
impl SRC_INC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_INC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_INC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `src_inc` writer - Source address increment"]
pub struct SRC_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_INC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `dst_size` reader - Destination data size"]
pub struct DST_SIZE_R(crate::FieldReader<u8, u8>);
impl DST_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DST_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dst_size` writer - Destination data size"]
pub struct DST_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `dst_inc` reader - Destination address increment"]
pub struct DST_INC_R(crate::FieldReader<u8, u8>);
impl DST_INC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DST_INC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_INC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dst_inc` writer - Destination address increment"]
pub struct DST_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_INC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The operating mode of the DMA cycle"]
    #[inline(always)]
    pub fn cycle_ctrl(&self) -> CYCLE_CTRL_R {
        CYCLE_CTRL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Controls if the chnl_useburst_set \\[C\\]
bit is set to a 1, when the controller is performing a peripheral scatter-gather and is completing a DMA cycle that uses the alternate data structure"]
    #[inline(always)]
    pub fn next_useburst(&self) -> NEXT_USEBURST_R {
        NEXT_USEBURST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:13 - Prior to the DMA cycle commencing, these bits represent the total number of DMA transfers that the DMA cycle contains. You must set these bits according to the size of DMA cycle that you require."]
    #[inline(always)]
    pub fn n_minus_1(&self) -> N_MINUS_1_R {
        N_MINUS_1_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bits 14:17 - Set these bits to control how many DMA transfers can occur before the controller rearbitrates"]
    #[inline(always)]
    pub fn r_power(&self) -> R_POWER_R {
        R_POWER_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:20 - Set the bits to control the state of HPROT\\[3:1\\]
when the controller reads the source data"]
    #[inline(always)]
    pub fn src_prot_ctrl(&self) -> SRC_PROT_CTRL_R {
        SRC_PROT_CTRL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Set the bits to control the state of HPROT\\[3:1\\]
when the controller writes the destination data"]
    #[inline(always)]
    pub fn dst_prot_ctrl(&self) -> DST_PROT_CTRL_R {
        DST_PROT_CTRL_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Source data size"]
    #[inline(always)]
    pub fn src_size(&self) -> SRC_SIZE_R {
        SRC_SIZE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Source address increment"]
    #[inline(always)]
    pub fn src_inc(&self) -> SRC_INC_R {
        SRC_INC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Destination data size"]
    #[inline(always)]
    pub fn dst_size(&self) -> DST_SIZE_R {
        DST_SIZE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Destination address increment"]
    #[inline(always)]
    pub fn dst_inc(&self) -> DST_INC_R {
        DST_INC_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The operating mode of the DMA cycle"]
    #[inline(always)]
    pub fn cycle_ctrl(&mut self) -> CYCLE_CTRL_W {
        CYCLE_CTRL_W { w: self }
    }
    #[doc = "Bit 3 - Controls if the chnl_useburst_set \\[C\\]
bit is set to a 1, when the controller is performing a peripheral scatter-gather and is completing a DMA cycle that uses the alternate data structure"]
    #[inline(always)]
    pub fn next_useburst(&mut self) -> NEXT_USEBURST_W {
        NEXT_USEBURST_W { w: self }
    }
    #[doc = "Bits 4:13 - Prior to the DMA cycle commencing, these bits represent the total number of DMA transfers that the DMA cycle contains. You must set these bits according to the size of DMA cycle that you require."]
    #[inline(always)]
    pub fn n_minus_1(&mut self) -> N_MINUS_1_W {
        N_MINUS_1_W { w: self }
    }
    #[doc = "Bits 14:17 - Set these bits to control how many DMA transfers can occur before the controller rearbitrates"]
    #[inline(always)]
    pub fn r_power(&mut self) -> R_POWER_W {
        R_POWER_W { w: self }
    }
    #[doc = "Bits 18:20 - Set the bits to control the state of HPROT\\[3:1\\]
when the controller reads the source data"]
    #[inline(always)]
    pub fn src_prot_ctrl(&mut self) -> SRC_PROT_CTRL_W {
        SRC_PROT_CTRL_W { w: self }
    }
    #[doc = "Bits 21:23 - Set the bits to control the state of HPROT\\[3:1\\]
when the controller writes the destination data"]
    #[inline(always)]
    pub fn dst_prot_ctrl(&mut self) -> DST_PROT_CTRL_W {
        DST_PROT_CTRL_W { w: self }
    }
    #[doc = "Bits 24:25 - Source data size"]
    #[inline(always)]
    pub fn src_size(&mut self) -> SRC_SIZE_W {
        SRC_SIZE_W { w: self }
    }
    #[doc = "Bits 26:27 - Source address increment"]
    #[inline(always)]
    pub fn src_inc(&mut self) -> SRC_INC_W {
        SRC_INC_W { w: self }
    }
    #[doc = "Bits 28:29 - Destination data size"]
    #[inline(always)]
    pub fn dst_size(&mut self) -> DST_SIZE_W {
        DST_SIZE_W { w: self }
    }
    #[doc = "Bits 30:31 - Destination address increment"]
    #[inline(always)]
    pub fn dst_inc(&mut self) -> DST_INC_W {
        DST_INC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Primary configuration for channel 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_chn_cfg_ch14](index.html) module"]
pub struct ALT_CHN_CFG_CH14_SPEC;
impl crate::RegisterSpec for ALT_CHN_CFG_CH14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alt_chn_cfg_ch14::R](R) reader structure"]
impl crate::Readable for ALT_CHN_CFG_CH14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alt_chn_cfg_ch14::W](W) writer structure"]
impl crate::Writable for ALT_CHN_CFG_CH14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALT_CHN_CFG_CH14 to value 0"]
impl crate::Resettable for ALT_CHN_CFG_CH14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
