#[doc = "Register `PKFB_PF8kDATA` reader"]
pub struct R(crate::R<PKFB_PF8KDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_PF8KDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_PF8KDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_PF8KDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_PF8kDATA` writer"]
pub struct W(crate::W<PKFB_PF8KDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_PF8KDATA_SPEC>;
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
impl From<crate::W<PKFB_PF8KDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_PF8KDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf8k_data_reg` reader - FIFO 8k Push/POP Data Register. In ringbuffer mode, \\[16\\]
is treated as SOP (start-of-packet) by autodrain logic"]
pub struct PF8K_DATA_REG_R(crate::FieldReader<u32, u32>);
impl PF8K_DATA_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PF8K_DATA_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_DATA_REG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_data_reg` writer - FIFO 8k Push/POP Data Register. In ringbuffer mode, \\[16\\]
is treated as SOP (start-of-packet) by autodrain logic"]
pub struct PF8K_DATA_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_DATA_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
#[doc = "Field `pf8k_push_eop` writer - FIFO 8k Push EOP (end of packet)"]
pub struct PF8K_PUSH_EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_PUSH_EOP_W<'a> {
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
            (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - FIFO 8k Push/POP Data Register. In ringbuffer mode, \\[16\\]
is treated as SOP (start-of-packet) by autodrain logic"]
    #[inline(always)]
    pub fn pf8k_data_reg(&self) -> PF8K_DATA_REG_R {
        PF8K_DATA_REG_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - FIFO 8k Push/POP Data Register. In ringbuffer mode, \\[16\\]
is treated as SOP (start-of-packet) by autodrain logic"]
    #[inline(always)]
    pub fn pf8k_data_reg(&mut self) -> PF8K_DATA_REG_W {
        PF8K_DATA_REG_W { w: self }
    }
    #[doc = "Bit 17 - FIFO 8k Push EOP (end of packet)"]
    #[inline(always)]
    pub fn pf8k_push_eop(&mut self) -> PF8K_PUSH_EOP_W {
        PF8K_PUSH_EOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO 8k Push/POP Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_pf8k_data](index.html) module"]
pub struct PKFB_PF8KDATA_SPEC;
impl crate::RegisterSpec for PKFB_PF8KDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_pf8k_data::R](R) reader structure"]
impl crate::Readable for PKFB_PF8KDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_pf8k_data::W](W) writer structure"]
impl crate::Writable for PKFB_PF8KDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_PF8kDATA to value 0"]
impl crate::Resettable for PKFB_PF8KDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
