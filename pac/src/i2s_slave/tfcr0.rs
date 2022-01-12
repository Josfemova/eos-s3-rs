#[doc = "Register `TFCR0` reader"]
pub struct R(crate::R<TFCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFCR0` writer"]
pub struct W(crate::W<TFCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFCR0_SPEC>;
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
impl From<crate::W<TFCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCHET` reader - Transmit Channel Empty Trigger. These bits program the trigger level in the TX FIFO at which the Empty Threshold Reached Interrupt is generated. Trigger Level = `TXCHET` `TXCHET` values: 0 to (`I2S_TX_FIFO_x – 1`) If an illegal value is programmed, these bits saturate to `(I2S_TX_FIFO_x – 1)`. The channel must be disabled prior to any changes in this value (that is, `TERx\\[0\\]
= 0`)."]
pub struct TXCHET_R(crate::FieldReader<u8, u8>);
impl TXCHET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXCHET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCHET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCHET` writer - Transmit Channel Empty Trigger. These bits program the trigger level in the TX FIFO at which the Empty Threshold Reached Interrupt is generated. Trigger Level = `TXCHET` `TXCHET` values: 0 to (`I2S_TX_FIFO_x – 1`) If an illegal value is programmed, these bits saturate to `(I2S_TX_FIFO_x – 1)`. The channel must be disabled prior to any changes in this value (that is, `TERx\\[0\\]
= 0`)."]
pub struct TXCHET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmit Channel Empty Trigger. These bits program the trigger level in the TX FIFO at which the Empty Threshold Reached Interrupt is generated. Trigger Level = `TXCHET` `TXCHET` values: 0 to (`I2S_TX_FIFO_x – 1`) If an illegal value is programmed, these bits saturate to `(I2S_TX_FIFO_x – 1)`. The channel must be disabled prior to any changes in this value (that is, `TERx\\[0\\]
= 0`)."]
    #[inline(always)]
    pub fn txchet(&self) -> TXCHET_R {
        TXCHET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit Channel Empty Trigger. These bits program the trigger level in the TX FIFO at which the Empty Threshold Reached Interrupt is generated. Trigger Level = `TXCHET` `TXCHET` values: 0 to (`I2S_TX_FIFO_x – 1`) If an illegal value is programmed, these bits saturate to `(I2S_TX_FIFO_x – 1)`. The channel must be disabled prior to any changes in this value (that is, `TERx\\[0\\]
= 0`)."]
    #[inline(always)]
    pub fn txchet(&mut self) -> TXCHET_W {
        TXCHET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfcr0](index.html) module"]
pub struct TFCR0_SPEC;
impl crate::RegisterSpec for TFCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfcr0::R](R) reader structure"]
impl crate::Readable for TFCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfcr0::W](W) writer structure"]
impl crate::Writable for TFCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFCR0 to value 0x04"]
impl crate::Resettable for TFCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
