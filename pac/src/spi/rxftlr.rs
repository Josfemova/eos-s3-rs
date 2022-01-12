#[doc = "Register `RXFTLR` reader"]
pub struct R(crate::R<RXFTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFTLR` writer"]
pub struct W(crate::W<RXFTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFTLR_SPEC>;
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
impl From<crate::W<RXFTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFT` reader - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt `ssi_rxf_intr`. The FIFO depth is configurable in the range 2-256. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For field decode, refer to Table 6-5."]
pub struct RFT_R(crate::FieldReader<u8, u8>);
impl RFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFT` writer - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt `ssi_rxf_intr`. The FIFO depth is configurable in the range 2-256. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For field decode, refer to Table 6-5."]
pub struct RFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt `ssi_rxf_intr`. The FIFO depth is configurable in the range 2-256. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For field decode, refer to Table 6-5."]
    #[inline(always)]
    pub fn rft(&self) -> RFT_R {
        RFT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt `ssi_rxf_intr`. The FIFO depth is configurable in the range 2-256. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For field decode, refer to Table 6-5."]
    #[inline(always)]
    pub fn rft(&mut self) -> RFT_W {
        RFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Threshold Level Register: This register controls the threshold value for the receive FIFO memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftlr](index.html) module"]
pub struct RXFTLR_SPEC;
impl crate::RegisterSpec for RXFTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxftlr::R](R) reader structure"]
impl crate::Readable for RXFTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxftlr::W](W) writer structure"]
impl crate::Writable for RXFTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXFTLR to value 0"]
impl crate::Resettable for RXFTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
