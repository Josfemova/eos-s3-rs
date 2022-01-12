#[doc = "Register `DMA_CTRL` reader"]
pub struct R(crate::R<DMA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CTRL` writer"]
pub struct W(crate::W<DMA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CTRL_SPEC>;
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
impl From<crate::W<DMA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_start` reader - write a 1: Enable, write a 0: no affect, reads dma_enb"]
pub struct DMA_START_R(crate::FieldReader<bool, bool>);
impl DMA_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_start` writer - write a 1: Enable, write a 0: no affect, reads dma_enb"]
pub struct DMA_START_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_START_W<'a> {
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
#[doc = "Field `dma_stop` reader - write a 1: Stop DMA and disable, clears DMA_DONE, write a 0: no affect, reads dma_done"]
pub struct DMA_STOP_R(crate::FieldReader<bool, bool>);
impl DMA_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_stop` writer - write a 1: Stop DMA and disable, clears DMA_DONE, write a 0: no affect, reads dma_done"]
pub struct DMA_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_STOP_W<'a> {
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
#[doc = "Field `dma_ahb_sel` reader - 0: DMA to AHB, 1: DMA to header register"]
pub struct DMA_AHB_SEL_R(crate::FieldReader<bool, bool>);
impl DMA_AHB_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_AHB_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_AHB_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_ahb_sel` writer - 0: DMA to AHB, 1: DMA to header register"]
pub struct DMA_AHB_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AHB_SEL_W<'a> {
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
#[doc = "Field `dma_hsel` reader - 1: AHB hsel is asserted, 0: not asserted"]
pub struct DMA_HSEL_R(crate::FieldReader<bool, bool>);
impl DMA_HSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_HSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_htrans_0` reader - 1: AHB htrans\\[0\\]
is asserted, 0: not asserted"]
pub struct DMA_HTRANS_0_R(crate::FieldReader<bool, bool>);
impl DMA_HTRANS_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HTRANS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_HTRANS_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_htrans_1` reader - 1: AHB htrans\\[1\\]
is asserted, 0: not asserted"]
pub struct DMA_HTRANS_1_R(crate::FieldReader<bool, bool>);
impl DMA_HTRANS_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HTRANS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_HTRANS_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_hready` reader - 1: AHB hready is asserted, 0: not asserted"]
pub struct DMA_HREADY_R(crate::FieldReader<bool, bool>);
impl DMA_HREADY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HREADY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_HREADY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_xfr_pending` reader - 1: DMA transfer is pending, 0: nothing pending"]
pub struct DMA_XFR_PENDING_R(crate::FieldReader<bool, bool>);
impl DMA_XFR_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_XFR_PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_XFR_PENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bridge_xfr_pending` reader - 1: AHB bridge transfer is pending, 0: nothing pending"]
pub struct BRIDGE_XFR_PENDING_R(crate::FieldReader<bool, bool>);
impl BRIDGE_XFR_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRIDGE_XFR_PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRIDGE_XFR_PENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - write a 1: Enable, write a 0: no affect, reads dma_enb"]
    #[inline(always)]
    pub fn dma_start(&self) -> DMA_START_R {
        DMA_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - write a 1: Stop DMA and disable, clears DMA_DONE, write a 0: no affect, reads dma_done"]
    #[inline(always)]
    pub fn dma_stop(&self) -> DMA_STOP_R {
        DMA_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0: DMA to AHB, 1: DMA to header register"]
    #[inline(always)]
    pub fn dma_ahb_sel(&self) -> DMA_AHB_SEL_R {
        DMA_AHB_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1: AHB hsel is asserted, 0: not asserted"]
    #[inline(always)]
    pub fn dma_hsel(&self) -> DMA_HSEL_R {
        DMA_HSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: AHB htrans\\[0\\]
is asserted, 0: not asserted"]
    #[inline(always)]
    pub fn dma_htrans_0(&self) -> DMA_HTRANS_0_R {
        DMA_HTRANS_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1: AHB htrans\\[1\\]
is asserted, 0: not asserted"]
    #[inline(always)]
    pub fn dma_htrans_1(&self) -> DMA_HTRANS_1_R {
        DMA_HTRANS_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1: AHB hready is asserted, 0: not asserted"]
    #[inline(always)]
    pub fn dma_hready(&self) -> DMA_HREADY_R {
        DMA_HREADY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: DMA transfer is pending, 0: nothing pending"]
    #[inline(always)]
    pub fn dma_xfr_pending(&self) -> DMA_XFR_PENDING_R {
        DMA_XFR_PENDING_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1: AHB bridge transfer is pending, 0: nothing pending"]
    #[inline(always)]
    pub fn bridge_xfr_pending(&self) -> BRIDGE_XFR_PENDING_R {
        BRIDGE_XFR_PENDING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write a 1: Enable, write a 0: no affect, reads dma_enb"]
    #[inline(always)]
    pub fn dma_start(&mut self) -> DMA_START_W {
        DMA_START_W { w: self }
    }
    #[doc = "Bit 1 - write a 1: Stop DMA and disable, clears DMA_DONE, write a 0: no affect, reads dma_done"]
    #[inline(always)]
    pub fn dma_stop(&mut self) -> DMA_STOP_W {
        DMA_STOP_W { w: self }
    }
    #[doc = "Bit 2 - 0: DMA to AHB, 1: DMA to header register"]
    #[inline(always)]
    pub fn dma_ahb_sel(&mut self) -> DMA_AHB_SEL_W {
        DMA_AHB_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctrl](index.html) module"]
pub struct DMA_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CTRL to value 0"]
impl crate::Resettable for DMA_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
