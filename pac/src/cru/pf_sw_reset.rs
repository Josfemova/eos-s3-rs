#[doc = "Register `PF_SW_RESET` reader"]
pub struct R(crate::R<PF_SW_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_SW_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_SW_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_SW_RESET` writer"]
pub struct W(crate::W<PF_SW_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_SW_RESET_SPEC>;
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
impl From<crate::W<PF_SW_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_SW_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF_FIFO_0_SW_RESET_A {
    #[doc = "1: Enable the software reset. FW need to disable it manually"]
    ENABLE = 1,
}
impl From<PF_FIFO_0_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: PF_FIFO_0_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PF_FIFO_0_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub struct PF_FIFO_0_SW_RESET_R(crate::FieldReader<bool, PF_FIFO_0_SW_RESET_A>);
impl PF_FIFO_0_SW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF_FIFO_0_SW_RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF_FIFO_0_SW_RESET_A> {
        match self.bits {
            true => Some(PF_FIFO_0_SW_RESET_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PF_FIFO_0_SW_RESET_A::ENABLE
    }
}
impl core::ops::Deref for PF_FIFO_0_SW_RESET_R {
    type Target = crate::FieldReader<bool, PF_FIFO_0_SW_RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF_FIFO_0_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub struct PF_FIFO_0_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_FIFO_0_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_FIFO_0_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_FIFO_0_SW_RESET_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_FIFO_1_SW_RESET_A = PF_FIFO_0_SW_RESET_A;
#[doc = "Field `PF_FIFO_1_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_FIFO_1_SW_RESET_R = PF_FIFO_0_SW_RESET_R;
#[doc = "Field `PF_FIFO_1_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub struct PF_FIFO_1_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_FIFO_1_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_FIFO_1_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_FIFO_1_SW_RESET_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_FIFO_2_SW_RESET_A = PF_FIFO_0_SW_RESET_A;
#[doc = "Field `PF_FIFO_2_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_FIFO_2_SW_RESET_R = PF_FIFO_0_SW_RESET_R;
#[doc = "Field `PF_FIFO_2_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub struct PF_FIFO_2_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_FIFO_2_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_FIFO_2_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_FIFO_2_SW_RESET_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_FIFO_8K_SW_RESET_A = PF_FIFO_0_SW_RESET_A;
#[doc = "Field `PF_FIFO_8k_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_FIFO_8K_SW_RESET_R = PF_FIFO_0_SW_RESET_R;
#[doc = "Field `PF_FIFO_8k_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub struct PF_FIFO_8K_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_FIFO_8K_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_FIFO_8K_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_FIFO_8K_SW_RESET_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually.(R08_P3 as well)"]
pub type PF_ASYNC_FIFO_0_SW_RESET_A = PF_FIFO_0_SW_RESET_A;
#[doc = "Field `PF_ASYNC_FIFO_0_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually.(R08_P3 as well)"]
pub type PF_ASYNC_FIFO_0_SW_RESET_R = PF_FIFO_0_SW_RESET_R;
#[doc = "Field `PF_ASYNC_FIFO_0_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually.(R08_P3 as well)"]
pub struct PF_ASYNC_FIFO_0_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_ASYNC_FIFO_0_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_ASYNC_FIFO_0_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_ASYNC_FIFO_0_SW_RESET_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_PERIPHERAL_SW_RESET_A = PF_FIFO_0_SW_RESET_A;
#[doc = "Field `PF_Peripheral_SW_RESET` reader - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub type PF_PERIPHERAL_SW_RESET_R = PF_FIFO_0_SW_RESET_R;
#[doc = "Field `PF_Peripheral_SW_RESET` writer - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
pub struct PF_PERIPHERAL_SW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_PERIPHERAL_SW_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_PERIPHERAL_SW_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the software reset. FW need to disable it manually"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF_PERIPHERAL_SW_RESET_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_0_sw_reset(&self) -> PF_FIFO_0_SW_RESET_R {
        PF_FIFO_0_SW_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_1_sw_reset(&self) -> PF_FIFO_1_SW_RESET_R {
        PF_FIFO_1_SW_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_2_sw_reset(&self) -> PF_FIFO_2_SW_RESET_R {
        PF_FIFO_2_SW_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_8k_sw_reset(&self) -> PF_FIFO_8K_SW_RESET_R {
        PF_FIFO_8K_SW_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1'b1 : Enable the Software Reset. FW need to disable it manually.(R08_P3 as well)"]
    #[inline(always)]
    pub fn pf_async_fifo_0_sw_reset(&self) -> PF_ASYNC_FIFO_0_SW_RESET_R {
        PF_ASYNC_FIFO_0_SW_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_peripheral_sw_reset(&self) -> PF_PERIPHERAL_SW_RESET_R {
        PF_PERIPHERAL_SW_RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_0_sw_reset(&mut self) -> PF_FIFO_0_SW_RESET_W {
        PF_FIFO_0_SW_RESET_W { w: self }
    }
    #[doc = "Bit 1 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_1_sw_reset(&mut self) -> PF_FIFO_1_SW_RESET_W {
        PF_FIFO_1_SW_RESET_W { w: self }
    }
    #[doc = "Bit 2 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_2_sw_reset(&mut self) -> PF_FIFO_2_SW_RESET_W {
        PF_FIFO_2_SW_RESET_W { w: self }
    }
    #[doc = "Bit 3 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_fifo_8k_sw_reset(&mut self) -> PF_FIFO_8K_SW_RESET_W {
        PF_FIFO_8K_SW_RESET_W { w: self }
    }
    #[doc = "Bit 4 - 1'b1 : Enable the Software Reset. FW need to disable it manually.(R08_P3 as well)"]
    #[inline(always)]
    pub fn pf_async_fifo_0_sw_reset(&mut self) -> PF_ASYNC_FIFO_0_SW_RESET_W {
        PF_ASYNC_FIFO_0_SW_RESET_W { w: self }
    }
    #[doc = "Bit 5 - 1'b1 : Enable the Software Reset. FW need to disable it manually."]
    #[inline(always)]
    pub fn pf_peripheral_sw_reset(&mut self) -> PF_PERIPHERAL_SW_RESET_W {
        PF_PERIPHERAL_SW_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet FIFO power domain Software Reset.Once Program the SW Reset Bit to 1, the corresponding reset will be asserted immediately. Once Program the SW Reset Bit to 0, the corresponding reset will be de-asserted synchronous even the corresponding clock is not running. (Turn off by Clock gating cell)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_sw_reset](index.html) module"]
pub struct PF_SW_RESET_SPEC;
impl crate::RegisterSpec for PF_SW_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_sw_reset::R](R) reader structure"]
impl crate::Readable for PF_SW_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_sw_reset::W](W) writer structure"]
impl crate::Writable for PF_SW_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_SW_RESET to value 0"]
impl crate::Resettable for PF_SW_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
