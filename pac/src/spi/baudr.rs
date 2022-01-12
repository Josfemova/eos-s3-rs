#[doc = "Register `BAUDR` reader"]
pub struct R(crate::R<BAUDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDR` writer"]
pub struct W(crate::W<BAUDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDR_SPEC>;
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
impl From<crate::W<BAUDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKDV_0` reader - From description of `SCKDV_15_1`: 'The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. ' `SCKDV` is broken into `SCK_DV_15_1` and `SCK_DV_0`."]
pub struct SCKDV_0_R(crate::FieldReader<bool, bool>);
impl SCKDV_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCKDV_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCKDV_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKDV_15_1` reader - SSI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: `Fsclk_out = Fssi_clk/SCKDV` where SCKDV is any even value between 2 and 65534. For example: for `Fssi_clk = 3.6864MHz` and `SCKDV` =2, `Fsclk_out = 3.6864/2 = 1.8432MHz`"]
pub struct SCKDV_15_1_R(crate::FieldReader<bool, bool>);
impl SCKDV_15_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCKDV_15_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCKDV_15_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKDV_15_1` writer - SSI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: `Fsclk_out = Fssi_clk/SCKDV` where SCKDV is any even value between 2 and 65534. For example: for `Fssi_clk = 3.6864MHz` and `SCKDV` =2, `Fsclk_out = 3.6864/2 = 1.8432MHz`"]
pub struct SCKDV_15_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKDV_15_1_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - From description of `SCKDV_15_1`: 'The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. ' `SCKDV` is broken into `SCK_DV_15_1` and `SCK_DV_0`."]
    #[inline(always)]
    pub fn sckdv_0(&self) -> SCKDV_0_R {
        SCKDV_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: `Fsclk_out = Fssi_clk/SCKDV` where SCKDV is any even value between 2 and 65534. For example: for `Fssi_clk = 3.6864MHz` and `SCKDV` =2, `Fsclk_out = 3.6864/2 = 1.8432MHz`"]
    #[inline(always)]
    pub fn sckdv_15_1(&self) -> SCKDV_15_1_R {
        SCKDV_15_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SSI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the following equation: `Fsclk_out = Fssi_clk/SCKDV` where SCKDV is any even value between 2 and 65534. For example: for `Fssi_clk = 3.6864MHz` and `SCKDV` =2, `Fsclk_out = 3.6864/2 = 1.8432MHz`"]
    #[inline(always)]
    pub fn sckdv_15_1(&mut self) -> SCKDV_15_1_W {
        SCKDV_15_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Select Register: This register derives the frequency of the serial clock that regulates the data transfer. It is impossible to write to this register when the SPI Master is enabled. The SPI Master is enabled and disabled by writing the SSIENR register (0x008).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baudr](index.html) module"]
pub struct BAUDR_SPEC;
impl crate::RegisterSpec for BAUDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [baudr::R](R) reader structure"]
impl crate::Readable for BAUDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baudr::W](W) writer structure"]
impl crate::Writable for BAUDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BAUDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
