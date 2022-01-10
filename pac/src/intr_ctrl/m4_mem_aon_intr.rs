#[doc = "Register `M4_MEM_AON_INTR` reader"]
pub struct R(crate::R<M4_MEM_AON_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_MEM_AON_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_MEM_AON_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_MEM_AON_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4_MEM_AON_INTR` writer"]
pub struct W(crate::W<M4_MEM_AON_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4_MEM_AON_INTR_SPEC>;
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
impl From<crate::W<M4_MEM_AON_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4_MEM_AON_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM3_AON_INTR0` reader - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR0_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR0` writer - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR0_W<'a> {
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
#[doc = "Field `MEM3_AON_INTR1` reader - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR1_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR1` writer - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR1_W<'a> {
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
#[doc = "Field `MEM3_AON_INTR2` reader - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR2_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR2` writer - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR2_W<'a> {
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
#[doc = "Field `MEM3_AON_INTR3` reader - Interrupt caused by a SRAM access (M4 SRAM segment 3 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR3_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR3` writer - Interrupt caused by a SRAM access (M4 SRAM segment 3 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM3_AON_INTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr0(&self) -> MEM3_AON_INTR0_R {
        MEM3_AON_INTR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr1(&self) -> MEM3_AON_INTR1_R {
        MEM3_AON_INTR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr2(&self) -> MEM3_AON_INTR2_R {
        MEM3_AON_INTR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt caused by a SRAM access (M4 SRAM segment 3 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr3(&self) -> MEM3_AON_INTR3_R {
        MEM3_AON_INTR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr0(&mut self) -> MEM3_AON_INTR0_W {
        MEM3_AON_INTR0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr1(&mut self) -> MEM3_AON_INTR1_W {
        MEM3_AON_INTR1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr2(&mut self) -> MEM3_AON_INTR2_W {
        MEM3_AON_INTR2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt caused by a SRAM access (M4 SRAM segment 3 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem3_aon_intr3(&mut self) -> MEM3_AON_INTR3_W {
        MEM3_AON_INTR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicator of detected SRAM access while shut down or sleep mode interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_mem_aon_intr](index.html) module"]
pub struct M4_MEM_AON_INTR_SPEC;
impl crate::RegisterSpec for M4_MEM_AON_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_mem_aon_intr::R](R) reader structure"]
impl crate::Readable for M4_MEM_AON_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4_mem_aon_intr::W](W) writer structure"]
impl crate::Writable for M4_MEM_AON_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4_MEM_AON_INTR to value 0"]
impl crate::Resettable for M4_MEM_AON_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
