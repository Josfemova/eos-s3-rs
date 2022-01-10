#[doc = "Register `PKFB_PF0POPCTL` reader"]
pub struct R(crate::R<PKFB_PF0POPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_PF0POPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_PF0POPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_PF0POPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_PF0POPCTL` writer"]
pub struct W(crate::W<PKFB_PF0POPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_PF0POPCTL_SPEC>;
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
impl From<crate::W<PKFB_PF0POPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_PF0POPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf0_pop_sleep_en` reader - Set this bit to enable sleep"]
pub struct PF0_POP_SLEEP_EN_R(crate::FieldReader<bool, bool>);
impl PF0_POP_SLEEP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_SLEEP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_POP_SLEEP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_sleep_en` writer - Set this bit to enable sleep"]
pub struct PF0_POP_SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_SLEEP_EN_W<'a> {
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
#[doc = "Select the type of sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_POP_SLEEP_TYPE_A {
    #[doc = "0: Select deep sleep as sleep type for the FIFO"]
    DEEP_SLEEP = 0,
    #[doc = "1: Select shutdown as the sleep type for the FIFO"]
    SHUT_DOWN = 1,
}
impl From<PF0_POP_SLEEP_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_POP_SLEEP_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_pop_sleep_type` reader - Select the type of sleep mode"]
pub struct PF0_POP_SLEEP_TYPE_R(crate::FieldReader<bool, PF0_POP_SLEEP_TYPE_A>);
impl PF0_POP_SLEEP_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_SLEEP_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_POP_SLEEP_TYPE_A {
        match self.bits {
            false => PF0_POP_SLEEP_TYPE_A::DEEP_SLEEP,
            true => PF0_POP_SLEEP_TYPE_A::SHUT_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        **self == PF0_POP_SLEEP_TYPE_A::DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_shut_down(&self) -> bool {
        **self == PF0_POP_SLEEP_TYPE_A::SHUT_DOWN
    }
}
impl core::ops::Deref for PF0_POP_SLEEP_TYPE_R {
    type Target = crate::FieldReader<bool, PF0_POP_SLEEP_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_sleep_type` writer - Select the type of sleep mode"]
pub struct PF0_POP_SLEEP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_SLEEP_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_SLEEP_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select deep sleep as sleep type for the FIFO"]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(PF0_POP_SLEEP_TYPE_A::DEEP_SLEEP)
    }
    #[doc = "Select shutdown as the sleep type for the FIFO"]
    #[inline(always)]
    pub fn shut_down(self) -> &'a mut W {
        self.variant(PF0_POP_SLEEP_TYPE_A::SHUT_DOWN)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Control whether the push interrupt for FIFO overflow is enabled or masked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_POP_INT_EN_OV_A {
    #[doc = "0: Mask the interrupt event"]
    MASK = 0,
    #[doc = "1: Unmask the interrupt event"]
    UNMASK = 1,
}
impl From<PF0_POP_INT_EN_OV_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_POP_INT_EN_OV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_pop_int_en_ov` reader - Control whether the push interrupt for FIFO overflow is enabled or masked"]
pub struct PF0_POP_INT_EN_OV_R(crate::FieldReader<bool, PF0_POP_INT_EN_OV_A>);
impl PF0_POP_INT_EN_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_INT_EN_OV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_POP_INT_EN_OV_A {
        match self.bits {
            false => PF0_POP_INT_EN_OV_A::MASK,
            true => PF0_POP_INT_EN_OV_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == PF0_POP_INT_EN_OV_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == PF0_POP_INT_EN_OV_A::UNMASK
    }
}
impl core::ops::Deref for PF0_POP_INT_EN_OV_R {
    type Target = crate::FieldReader<bool, PF0_POP_INT_EN_OV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_int_en_ov` writer - Control whether the push interrupt for FIFO overflow is enabled or masked"]
pub struct PF0_POP_INT_EN_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_EN_OV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_INT_EN_OV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PF0_POP_INT_EN_OV_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PF0_POP_INT_EN_OV_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Control whether the push interrupt for FIFO threshold is enabled or masked"]
pub type PF0_POP_INT_EN_THRESH_A = PF0_POP_INT_EN_OV_A;
#[doc = "Field `pf0_pop_int_en_thresh` reader - Control whether the push interrupt for FIFO threshold is enabled or masked"]
pub type PF0_POP_INT_EN_THRESH_R = PF0_POP_INT_EN_OV_R;
#[doc = "Field `pf0_pop_int_en_thresh` writer - Control whether the push interrupt for FIFO threshold is enabled or masked"]
pub struct PF0_POP_INT_EN_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_EN_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_INT_EN_THRESH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PF0_POP_INT_EN_THRESH_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PF0_POP_INT_EN_THRESH_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Control whether the push interrupt for pop on SRAM sleep is enabled or masked"]
pub type PF0_POP_INT_EN_SRAM_SLEEP_A = PF0_POP_INT_EN_OV_A;
#[doc = "Field `pf0_pop_int_en_sram_sleep` reader - Control whether the push interrupt for pop on SRAM sleep is enabled or masked"]
pub type PF0_POP_INT_EN_SRAM_SLEEP_R = PF0_POP_INT_EN_OV_R;
#[doc = "Field `pf0_pop_int_en_sram_sleep` writer - Control whether the push interrupt for pop on SRAM sleep is enabled or masked"]
pub struct PF0_POP_INT_EN_SRAM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_EN_SRAM_SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_INT_EN_SRAM_SLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt event"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(PF0_POP_INT_EN_SRAM_SLEEP_A::MASK)
    }
    #[doc = "Unmask the interrupt event"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(PF0_POP_INT_EN_SRAM_SLEEP_A::UNMASK)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `pf0_pop_thresh` reader - POP counter threshold (x32 count)"]
pub struct PF0_POP_THRESH_R(crate::FieldReader<u16, u16>);
impl PF0_POP_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PF0_POP_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF0_POP_THRESH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_thresh` writer - POP counter threshold (x32 count)"]
pub struct PF0_POP_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to enable sleep"]
    #[inline(always)]
    pub fn pf0_pop_sleep_en(&self) -> PF0_POP_SLEEP_EN_R {
        PF0_POP_SLEEP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select the type of sleep mode"]
    #[inline(always)]
    pub fn pf0_pop_sleep_type(&self) -> PF0_POP_SLEEP_TYPE_R {
        PF0_POP_SLEEP_TYPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control whether the push interrupt for FIFO overflow is enabled or masked"]
    #[inline(always)]
    pub fn pf0_pop_int_en_ov(&self) -> PF0_POP_INT_EN_OV_R {
        PF0_POP_INT_EN_OV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control whether the push interrupt for FIFO threshold is enabled or masked"]
    #[inline(always)]
    pub fn pf0_pop_int_en_thresh(&self) -> PF0_POP_INT_EN_THRESH_R {
        PF0_POP_INT_EN_THRESH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control whether the push interrupt for pop on SRAM sleep is enabled or masked"]
    #[inline(always)]
    pub fn pf0_pop_int_en_sram_sleep(&self) -> PF0_POP_INT_EN_SRAM_SLEEP_R {
        PF0_POP_INT_EN_SRAM_SLEEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - POP counter threshold (x32 count)"]
    #[inline(always)]
    pub fn pf0_pop_thresh(&self) -> PF0_POP_THRESH_R {
        PF0_POP_THRESH_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable sleep"]
    #[inline(always)]
    pub fn pf0_pop_sleep_en(&mut self) -> PF0_POP_SLEEP_EN_W {
        PF0_POP_SLEEP_EN_W { w: self }
    }
    #[doc = "Bit 1 - Select the type of sleep mode"]
    #[inline(always)]
    pub fn pf0_pop_sleep_type(&mut self) -> PF0_POP_SLEEP_TYPE_W {
        PF0_POP_SLEEP_TYPE_W { w: self }
    }
    #[doc = "Bit 2 - Control whether the push interrupt for FIFO overflow is enabled or masked"]
    #[inline(always)]
    pub fn pf0_pop_int_en_ov(&mut self) -> PF0_POP_INT_EN_OV_W {
        PF0_POP_INT_EN_OV_W { w: self }
    }
    #[doc = "Bit 3 - Control whether the push interrupt for FIFO threshold is enabled or masked"]
    #[inline(always)]
    pub fn pf0_pop_int_en_thresh(&mut self) -> PF0_POP_INT_EN_THRESH_W {
        PF0_POP_INT_EN_THRESH_W { w: self }
    }
    #[doc = "Bit 4 - Control whether the push interrupt for pop on SRAM sleep is enabled or masked"]
    #[inline(always)]
    pub fn pf0_pop_int_en_sram_sleep(&mut self) -> PF0_POP_INT_EN_SRAM_SLEEP_W {
        PF0_POP_INT_EN_SRAM_SLEEP_W { w: self }
    }
    #[doc = "Bits 16:24 - POP counter threshold (x32 count)"]
    #[inline(always)]
    pub fn pf0_pop_thresh(&mut self) -> PF0_POP_THRESH_W {
        PF0_POP_THRESH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO 0 Pop Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_pf0popctl](index.html) module"]
pub struct PKFB_PF0POPCTL_SPEC;
impl crate::RegisterSpec for PKFB_PF0POPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_pf0popctl::R](R) reader structure"]
impl crate::Readable for PKFB_PF0POPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_pf0popctl::W](W) writer structure"]
impl crate::Writable for PKFB_PF0POPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_PF0POPCTL to value 0"]
impl crate::Resettable for PKFB_PF0POPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
