#[doc = "Register `PDM_CORE_CONFIG` reader"]
pub struct R(crate::R<PDM_CORE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_CORE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_CORE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_CORE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_CORE_CONFIG` writer"]
pub struct W(crate::W<PDM_CORE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_CORE_CONFIG_SPEC>;
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
impl From<crate::W<PDM_CORE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_CORE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDM_CORE_CONFIG` reader - Set to enable PDM core"]
pub struct PDM_CORE_CONFIG_R(crate::FieldReader<bool, bool>);
impl PDM_CORE_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDM_CORE_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM_CORE_CONFIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_CORE_CONFIG` writer - Set to enable PDM core"]
pub struct PDM_CORE_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_CORE_CONFIG_W<'a> {
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
#[doc = "Field `SOFT_MUTE` reader - Set to enable PDM Soft mute"]
pub struct SOFT_MUTE_R(crate::FieldReader<bool, bool>);
impl SOFT_MUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_MUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_MUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_MUTE` writer - Set to enable PDM Soft mute"]
pub struct SOFT_MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_MUTE_W<'a> {
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
#[doc = "Select which divider to use in sampler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_MODE_A {
    #[doc = "0: Use PDM_LEFT in sampler"]
    LEFT = 0,
    #[doc = "1: Use PDM_RIGHT in sampler"]
    RIGHT = 1,
}
impl From<DIV_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DIV_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV_MODE` reader - Select which divider to use in sampler"]
pub struct DIV_MODE_R(crate::FieldReader<bool, DIV_MODE_A>);
impl DIV_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIV_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV_MODE_A {
        match self.bits {
            false => DIV_MODE_A::LEFT,
            true => DIV_MODE_A::RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == DIV_MODE_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        **self == DIV_MODE_A::RIGHT
    }
}
impl core::ops::Deref for DIV_MODE_R {
    type Target = crate::FieldReader<bool, DIV_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_MODE` writer - Select which divider to use in sampler"]
pub struct DIV_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use PDM_LEFT in sampler"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(DIV_MODE_A::LEFT)
    }
    #[doc = "Use PDM_RIGHT in sampler"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(DIV_MODE_A::RIGHT)
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
#[doc = "Field `S_CYCLES` reader - Set number of PDM_CLK during gain setting changes or soft mute"]
pub struct S_CYCLES_R(crate::FieldReader<u8, u8>);
impl S_CYCLES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        S_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_CYCLES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_CYCLES` writer - Set number of PDM_CLK during gain setting changes or soft mute"]
pub struct S_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> S_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `HP_GAIN` reader - Adjust high pass filter coefficients"]
pub struct HP_GAIN_R(crate::FieldReader<u8, u8>);
impl HP_GAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HP_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HP_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HP_GAIN` writer - Adjust high pass filter coefficients"]
pub struct HP_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "Field `ADCHPD` reader - Set to disable high pass filter"]
pub struct ADCHPD_R(crate::FieldReader<bool, bool>);
impl ADCHPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADCHPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCHPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCHPD` writer - Set to disable high pass filter"]
pub struct ADCHPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCHPD_W<'a> {
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
#[doc = "Field `M_CLK_DIV` reader - PDM_CLK frquency divisor"]
pub struct M_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl M_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_CLK_DIV` writer - PDM_CLK frquency divisor"]
pub struct M_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> M_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `SINC_RATE` reader - SINC decimation rate"]
pub struct SINC_RATE_R(crate::FieldReader<u8, u8>);
impl SINC_RATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SINC_RATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINC_RATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINC_RATE` writer - SINC decimation rate"]
pub struct SINC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x7f << 13)) | ((value as u32 & 0x7f) << 13);
        self.w
    }
}
#[doc = "Field `PGA_L` reader - Left channel PGA gain"]
pub struct PGA_L_R(crate::FieldReader<u8, u8>);
impl PGA_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PGA_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGA_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGA_L` writer - Left channel PGA gain"]
pub struct PGA_L_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `PGA_R` reader - Right channel PGA gain"]
pub struct PGA_R_R(crate::FieldReader<u8, u8>);
impl PGA_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PGA_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGA_R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGA_R` writer - Right channel PGA gain"]
pub struct PGA_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `DMICK_DLY` reader - Input data sampling with PDM clock cycle delay"]
pub struct DMICK_DLY_R(crate::FieldReader<bool, bool>);
impl DMICK_DLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMICK_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMICK_DLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMICK_DLY` writer - Input data sampling with PDM clock cycle delay"]
pub struct DMICK_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMICK_DLY_W<'a> {
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
#[doc = "Field `DIV_WD_MODE` reader - Status IN detection windows"]
pub struct DIV_WD_MODE_R(crate::FieldReader<bool, bool>);
impl DIV_WD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIV_WD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_WD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_WD_MODE` writer - Status IN detection windows"]
pub struct DIV_WD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_WD_MODE_W<'a> {
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
    #[doc = "Bit 0 - Set to enable PDM core"]
    #[inline(always)]
    pub fn pdm_core_config(&self) -> PDM_CORE_CONFIG_R {
        PDM_CORE_CONFIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to enable PDM Soft mute"]
    #[inline(always)]
    pub fn soft_mute(&self) -> SOFT_MUTE_R {
        SOFT_MUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select which divider to use in sampler"]
    #[inline(always)]
    pub fn div_mode(&self) -> DIV_MODE_R {
        DIV_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Set number of PDM_CLK during gain setting changes or soft mute"]
    #[inline(always)]
    pub fn s_cycles(&self) -> S_CYCLES_R {
        S_CYCLES_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:9 - Adjust high pass filter coefficients"]
    #[inline(always)]
    pub fn hp_gain(&self) -> HP_GAIN_R {
        HP_GAIN_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Set to disable high pass filter"]
    #[inline(always)]
    pub fn adchpd(&self) -> ADCHPD_R {
        ADCHPD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - PDM_CLK frquency divisor"]
    #[inline(always)]
    pub fn m_clk_div(&self) -> M_CLK_DIV_R {
        M_CLK_DIV_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:19 - SINC decimation rate"]
    #[inline(always)]
    pub fn sinc_rate(&self) -> SINC_RATE_R {
        SINC_RATE_R::new(((self.bits >> 13) & 0x7f) as u8)
    }
    #[doc = "Bits 20:24 - Left channel PGA gain"]
    #[inline(always)]
    pub fn pga_l(&self) -> PGA_L_R {
        PGA_L_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Right channel PGA gain"]
    #[inline(always)]
    pub fn pga_r(&self) -> PGA_R_R {
        PGA_R_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Input data sampling with PDM clock cycle delay"]
    #[inline(always)]
    pub fn dmick_dly(&self) -> DMICK_DLY_R {
        DMICK_DLY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status IN detection windows"]
    #[inline(always)]
    pub fn div_wd_mode(&self) -> DIV_WD_MODE_R {
        DIV_WD_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable PDM core"]
    #[inline(always)]
    pub fn pdm_core_config(&mut self) -> PDM_CORE_CONFIG_W {
        PDM_CORE_CONFIG_W { w: self }
    }
    #[doc = "Bit 1 - Set to enable PDM Soft mute"]
    #[inline(always)]
    pub fn soft_mute(&mut self) -> SOFT_MUTE_W {
        SOFT_MUTE_W { w: self }
    }
    #[doc = "Bit 2 - Select which divider to use in sampler"]
    #[inline(always)]
    pub fn div_mode(&mut self) -> DIV_MODE_W {
        DIV_MODE_W { w: self }
    }
    #[doc = "Bits 3:5 - Set number of PDM_CLK during gain setting changes or soft mute"]
    #[inline(always)]
    pub fn s_cycles(&mut self) -> S_CYCLES_W {
        S_CYCLES_W { w: self }
    }
    #[doc = "Bits 6:9 - Adjust high pass filter coefficients"]
    #[inline(always)]
    pub fn hp_gain(&mut self) -> HP_GAIN_W {
        HP_GAIN_W { w: self }
    }
    #[doc = "Bit 10 - Set to disable high pass filter"]
    #[inline(always)]
    pub fn adchpd(&mut self) -> ADCHPD_W {
        ADCHPD_W { w: self }
    }
    #[doc = "Bits 11:12 - PDM_CLK frquency divisor"]
    #[inline(always)]
    pub fn m_clk_div(&mut self) -> M_CLK_DIV_W {
        M_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 13:19 - SINC decimation rate"]
    #[inline(always)]
    pub fn sinc_rate(&mut self) -> SINC_RATE_W {
        SINC_RATE_W { w: self }
    }
    #[doc = "Bits 20:24 - Left channel PGA gain"]
    #[inline(always)]
    pub fn pga_l(&mut self) -> PGA_L_W {
        PGA_L_W { w: self }
    }
    #[doc = "Bits 25:29 - Right channel PGA gain"]
    #[inline(always)]
    pub fn pga_r(&mut self) -> PGA_R_W {
        PGA_R_W { w: self }
    }
    #[doc = "Bit 30 - Input data sampling with PDM clock cycle delay"]
    #[inline(always)]
    pub fn dmick_dly(&mut self) -> DMICK_DLY_W {
        DMICK_DLY_W { w: self }
    }
    #[doc = "Bit 31 - Status IN detection windows"]
    #[inline(always)]
    pub fn div_wd_mode(&mut self) -> DIV_WD_MODE_W {
        DIV_WD_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM2PCM core configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_core_config](index.html) module"]
pub struct PDM_CORE_CONFIG_SPEC;
impl crate::RegisterSpec for PDM_CORE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_core_config::R](R) reader structure"]
impl crate::Readable for PDM_CORE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_core_config::W](W) writer structure"]
impl crate::Writable for PDM_CORE_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDM_CORE_CONFIG to value 0x1082_66c9"]
impl crate::Resettable for PDM_CORE_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1082_66c9
    }
}
