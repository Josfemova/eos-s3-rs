#[doc = "Register `DMADEBUGCTL1` reader"]
pub struct R(crate::R<DMADEBUGCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMADEBUGCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMADEBUGCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMADEBUGCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMADEBUGCTL1` writer"]
pub struct W(crate::W<DMADEBUGCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMADEBUGCTL1_SPEC>;
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
impl From<crate::W<DMADEBUGCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMADEBUGCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DmaClear` reader - Set bit to Reset the DMA engineer. Firmware needs to clear it before kick off the new DMA Transfer. Need to do a dummy Read on this SFR after program this bit"]
pub struct DMACLEAR_R(crate::FieldReader<bool, bool>);
impl DMACLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMACLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMACLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DmaClear` writer - Set bit to Reset the DMA engineer. Firmware needs to clear it before kick off the new DMA Transfer. Need to do a dummy Read on this SFR after program this bit"]
pub struct DMACLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACLEAR_W<'a> {
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
    #[doc = "Bit 0 - Set bit to Reset the DMA engineer. Firmware needs to clear it before kick off the new DMA Transfer. Need to do a dummy Read on this SFR after program this bit"]
    #[inline(always)]
    pub fn dma_clear(&self) -> DMACLEAR_R {
        DMACLEAR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set bit to Reset the DMA engineer. Firmware needs to clear it before kick off the new DMA Transfer. Need to do a dummy Read on this SFR after program this bit"]
    #[inline(always)]
    pub fn dma_clear(&mut self) -> DMACLEAR_W {
        DMACLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bit to Reset the DMA engineer. Firmware needs to clear it before kick off the new DMA Transfer. Need to do a dummy Read on this SFR after program this bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmadebugctl1](index.html) module"]
pub struct DMADEBUGCTL1_SPEC;
impl crate::RegisterSpec for DMADEBUGCTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmadebugctl1::R](R) reader structure"]
impl crate::Readable for DMADEBUGCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmadebugctl1::W](W) writer structure"]
impl crate::Writable for DMADEBUGCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMADEBUGCTL1 to value 0"]
impl crate::Resettable for DMADEBUGCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
