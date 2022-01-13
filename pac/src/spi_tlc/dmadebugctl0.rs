#[doc = "Register `DMADEBUGCTL0` reader"]
pub struct R(crate::R<DMADEBUGCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMADEBUGCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMADEBUGCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMADEBUGCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMADEBUGCTL0` writer"]
pub struct W(crate::W<DMADEBUGCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMADEBUGCTL0_SPEC>;
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
impl From<crate::W<DMADEBUGCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMADEBUGCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DmaFifoClear` reader - Set bit to clear the DMA FIFO. Firmware can only set this bit to 1 after set DmaClear bit to 1 and Program this bit to 0 after clear DmaClear bit"]
pub struct DMAFIFOCLEAR_R(crate::FieldReader<bool, bool>);
impl DMAFIFOCLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAFIFOCLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAFIFOCLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DmaFifoClear` writer - Set bit to clear the DMA FIFO. Firmware can only set this bit to 1 after set DmaClear bit to 1 and Program this bit to 0 after clear DmaClear bit"]
pub struct DMAFIFOCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAFIFOCLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set bit to clear the DMA FIFO. Firmware can only set this bit to 1 after set DmaClear bit to 1 and Program this bit to 0 after clear DmaClear bit"]
    #[inline(always)]
    pub fn dma_fifo_clear(&self) -> DMAFIFOCLEAR_R {
        DMAFIFOCLEAR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set bit to clear the DMA FIFO. Firmware can only set this bit to 1 after set DmaClear bit to 1 and Program this bit to 0 after clear DmaClear bit"]
    #[inline(always)]
    pub fn dma_fifo_clear(&mut self) -> DMAFIFOCLEAR_W {
        DMAFIFOCLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bit to clear the DMA FIFO. Firmware can only set this bit to 1 after set DmaClear bit to 1 and Program this bit to 0 after clear DmaClear bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmadebugctl0](index.html) module"]
pub struct DMADEBUGCTL0_SPEC;
impl crate::RegisterSpec for DMADEBUGCTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmadebugctl0::R](R) reader structure"]
impl crate::Readable for DMADEBUGCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmadebugctl0::W](W) writer structure"]
impl crate::Writable for DMADEBUGCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMADEBUGCTL0 to value 0"]
impl crate::Resettable for DMADEBUGCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
