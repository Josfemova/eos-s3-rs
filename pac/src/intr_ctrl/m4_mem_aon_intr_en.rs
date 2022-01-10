#[doc = "Register `M4_MEM_AON_INTR_EN` reader"]
pub struct R(crate::R<M4_MEM_AON_INTR_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_MEM_AON_INTR_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_MEM_AON_INTR_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_MEM_AON_INTR_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4_MEM_AON_INTR_EN` writer"]
pub struct W(crate::W<M4_MEM_AON_INTR_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4_MEM_AON_INTR_EN_SPEC>;
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
impl From<crate::W<M4_MEM_AON_INTR_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4_MEM_AON_INTR_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM3_AON_INTR0_EN` reader - Interrupt enable (M4 SRAM segment 0 32KB_0)"]
pub struct MEM3_AON_INTR0_EN_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR0_EN` writer - Interrupt enable (M4 SRAM segment 0 32KB_0)"]
pub struct MEM3_AON_INTR0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR0_EN_W<'a> {
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
#[doc = "Field `MEM3_AON_INTR1_EN` reader - Interrupt enable (M4 SRAM segment 1 32KB_0)"]
pub struct MEM3_AON_INTR1_EN_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR1_EN` writer - Interrupt enable (M4 SRAM segment 1 32KB_0)"]
pub struct MEM3_AON_INTR1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR1_EN_W<'a> {
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
#[doc = "Field `MEM3_AON_INTR2_EN` reader - Interrupt enable (M4 SRAM segment 2 32KB_0)"]
pub struct MEM3_AON_INTR2_EN_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR2_EN` writer - Interrupt enable (M4 SRAM segment 2 32KB_0)"]
pub struct MEM3_AON_INTR2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR2_EN_W<'a> {
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
#[doc = "Field `MEM3_AON_INTR3_EN` reader - Interrupt enable (M4 SRAM segment 3 32KB_0)"]
pub struct MEM3_AON_INTR3_EN_R(crate::FieldReader<bool, bool>);
impl MEM3_AON_INTR3_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_AON_INTR3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_AON_INTR3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_AON_INTR3_EN` writer - Interrupt enable (M4 SRAM segment 3 32KB_0)"]
pub struct MEM3_AON_INTR3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_AON_INTR3_EN_W<'a> {
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
    #[doc = "Bit 0 - Interrupt enable (M4 SRAM segment 0 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr0_en(&self) -> MEM3_AON_INTR0_EN_R {
        MEM3_AON_INTR0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable (M4 SRAM segment 1 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr1_en(&self) -> MEM3_AON_INTR1_EN_R {
        MEM3_AON_INTR1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable (M4 SRAM segment 2 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr2_en(&self) -> MEM3_AON_INTR2_EN_R {
        MEM3_AON_INTR2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable (M4 SRAM segment 3 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr3_en(&self) -> MEM3_AON_INTR3_EN_R {
        MEM3_AON_INTR3_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable (M4 SRAM segment 0 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr0_en(&mut self) -> MEM3_AON_INTR0_EN_W {
        MEM3_AON_INTR0_EN_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable (M4 SRAM segment 1 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr1_en(&mut self) -> MEM3_AON_INTR1_EN_W {
        MEM3_AON_INTR1_EN_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable (M4 SRAM segment 2 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr2_en(&mut self) -> MEM3_AON_INTR2_EN_W {
        MEM3_AON_INTR2_EN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable (M4 SRAM segment 3 32KB_0)"]
    #[inline(always)]
    pub fn mem3_aon_intr3_en(&mut self) -> MEM3_AON_INTR3_EN_W {
        MEM3_AON_INTR3_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable for SRAM access while in shut down or deep sleep modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_mem_aon_intr_en](index.html) module"]
pub struct M4_MEM_AON_INTR_EN_SPEC;
impl crate::RegisterSpec for M4_MEM_AON_INTR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_mem_aon_intr_en::R](R) reader structure"]
impl crate::Readable for M4_MEM_AON_INTR_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4_mem_aon_intr_en::W](W) writer structure"]
impl crate::Writable for M4_MEM_AON_INTR_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4_MEM_AON_INTR_EN to value 0"]
impl crate::Resettable for M4_MEM_AON_INTR_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
