#[doc = "Register `I2S_CONFIG` reader"]
pub struct R(crate::R<I2S_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_CONFIG` writer"]
pub struct W(crate::W<I2S_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_CONFIG_SPEC>;
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
impl From<crate::W<I2S_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_LRCDIV` reader - I2S_CLK divisor for WD_CLK generator"]
pub struct I2S_LRCDIV_R(crate::FieldReader<u16, u16>);
impl I2S_LRCDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        I2S_LRCDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_LRCDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_LRCDIV` writer - I2S_CLK divisor for WD_CLK generator"]
pub struct I2S_LRCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_LRCDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `I2S_BLKDIV` reader - I2S_MASTER_CLK divisor for I2S_CLK generator"]
pub struct I2S_BLKDIV_R(crate::FieldReader<u8, u8>);
impl I2S_BLKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_BLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_BLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_BLKDIV` writer - I2S_MASTER_CLK divisor for I2S_CLK generator"]
pub struct I2S_BLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_BLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `I2S_CLK_INV` reader - Set to activate inverting"]
pub struct I2S_CLK_INV_R(crate::FieldReader<bool, bool>);
impl I2S_CLK_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_CLK_INV` writer - Set to activate inverting"]
pub struct I2S_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CLK_INV_W<'a> {
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
#[doc = "Field `I2S_IWL` reader - Input sample data bit shift"]
pub struct I2S_IWL_R(crate::FieldReader<u8, u8>);
impl I2S_IWL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2S_IWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_IWL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S_IWL` writer - Input sample data bit shift"]
pub struct I2S_IWL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_IWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - I2S_CLK divisor for WD_CLK generator"]
    #[inline(always)]
    pub fn i2s_lrcdiv(&self) -> I2S_LRCDIV_R {
        I2S_LRCDIV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:17 - I2S_MASTER_CLK divisor for I2S_CLK generator"]
    #[inline(always)]
    pub fn i2s_blkdiv(&self) -> I2S_BLKDIV_R {
        I2S_BLKDIV_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 18 - Set to activate inverting"]
    #[inline(always)]
    pub fn i2s_clk_inv(&self) -> I2S_CLK_INV_R {
        I2S_CLK_INV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - Input sample data bit shift"]
    #[inline(always)]
    pub fn i2s_iwl(&self) -> I2S_IWL_R {
        I2S_IWL_R::new(((self.bits >> 19) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - I2S_CLK divisor for WD_CLK generator"]
    #[inline(always)]
    pub fn i2s_lrcdiv(&mut self) -> I2S_LRCDIV_W {
        I2S_LRCDIV_W { w: self }
    }
    #[doc = "Bits 12:17 - I2S_MASTER_CLK divisor for I2S_CLK generator"]
    #[inline(always)]
    pub fn i2s_blkdiv(&mut self) -> I2S_BLKDIV_W {
        I2S_BLKDIV_W { w: self }
    }
    #[doc = "Bit 18 - Set to activate inverting"]
    #[inline(always)]
    pub fn i2s_clk_inv(&mut self) -> I2S_CLK_INV_W {
        I2S_CLK_INV_W { w: self }
    }
    #[doc = "Bits 19:20 - Input sample data bit shift"]
    #[inline(always)]
    pub fn i2s_iwl(&mut self) -> I2S_IWL_W {
        I2S_IWL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S master configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_config](index.html) module"]
pub struct I2S_CONFIG_SPEC;
impl crate::RegisterSpec for I2S_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_config::R](R) reader structure"]
impl crate::Readable for I2S_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_config::W](W) writer structure"]
impl crate::Writable for I2S_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_CONFIG to value 0x2040"]
impl crate::Resettable for I2S_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2040
    }
}
