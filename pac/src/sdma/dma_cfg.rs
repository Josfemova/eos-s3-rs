#[doc = "Register `DMA_CFG` writer"]
pub struct W(crate::W<DMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CFG_SPEC>;
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
impl From<crate::W<DMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `master_enable` writer - Set to enable the controller"]
pub struct MASTER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_ENABLE_W<'a> {
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
#[doc = "Field `chnl_prot_ctrl` writer - Sets the AHB-Lite protection by controlling the HPROT\\[3:1\\]
signal levels as follows: \n \n Bit \\[7\\]
Controls HPROT\\[3\\]
to indicate if a cacheable access is occurring. \n \n Bit \\[6\\]
Controls HPROT\\[2\\]
to indicate if a bufferable access is occurring. \n \n Bit \\[5\\]
Controls HPROT\\[1\\]
to indicate if a privileged access is occurring. \n \n Note\n \n When bit \\[n\\]
= 1 then the corresponding HPROT is HIGH. \n \n When bit \\[n\\]
= 0 then the corresponding HPROT is LOW."]
pub struct CHNL_PROT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_PROT_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable the controller"]
    #[inline(always)]
    pub fn master_enable(&mut self) -> MASTER_ENABLE_W {
        MASTER_ENABLE_W { w: self }
    }
    #[doc = "Bits 5:7 - Sets the AHB-Lite protection by controlling the HPROT\\[3:1\\]
signal levels as follows: \n \n Bit \\[7\\]
Controls HPROT\\[3\\]
to indicate if a cacheable access is occurring. \n \n Bit \\[6\\]
Controls HPROT\\[2\\]
to indicate if a bufferable access is occurring. \n \n Bit \\[5\\]
Controls HPROT\\[1\\]
to indicate if a privileged access is occurring. \n \n Note\n \n When bit \\[n\\]
= 1 then the corresponding HPROT is HIGH. \n \n When bit \\[n\\]
= 0 then the corresponding HPROT is LOW."]
    #[inline(always)]
    pub fn chnl_prot_ctrl(&mut self) -> CHNL_PROT_CTRL_W {
        CHNL_PROT_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA configuration register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_cfg](index.html) module"]
pub struct DMA_CFG_SPEC;
impl crate::RegisterSpec for DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_cfg::W](W) writer structure"]
impl crate::Writable for DMA_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CFG to value 0"]
impl crate::Resettable for DMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
