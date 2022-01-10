#[doc = "Register `SOFTWARE_INTR_2_EN_AP` reader"]
pub struct R(crate::R<SOFTWARE_INTR_2_EN_AP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTWARE_INTR_2_EN_AP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTWARE_INTR_2_EN_AP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTWARE_INTR_2_EN_AP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTWARE_INTR_2_EN_AP` writer"]
pub struct W(crate::W<SOFTWARE_INTR_2_EN_AP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTWARE_INTR_2_EN_AP_SPEC>;
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
impl From<crate::W<SOFTWARE_INTR_2_EN_AP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTWARE_INTR_2_EN_AP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_INTR_2_EN_AP` reader - Software interrupt 2 enable for AP"]
pub struct SW_INTR_2_EN_AP_R(crate::FieldReader<bool, bool>);
impl SW_INTR_2_EN_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_INTR_2_EN_AP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_INTR_2_EN_AP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_INTR_2_EN_AP` writer - Software interrupt 2 enable for AP"]
pub struct SW_INTR_2_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_INTR_2_EN_AP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Software interrupt 2 enable for AP"]
    #[inline(always)]
    pub fn sw_intr_2_en_ap(&self) -> SW_INTR_2_EN_AP_R {
        SW_INTR_2_EN_AP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt 2 enable for AP"]
    #[inline(always)]
    pub fn sw_intr_2_en_ap(&mut self) -> SW_INTR_2_EN_AP_W {
        SW_INTR_2_EN_AP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose Software interrupt 2 enable for AP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [software_intr_2_en_ap](index.html) module"]
pub struct SOFTWARE_INTR_2_EN_AP_SPEC;
impl crate::RegisterSpec for SOFTWARE_INTR_2_EN_AP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [software_intr_2_en_ap::R](R) reader structure"]
impl crate::Readable for SOFTWARE_INTR_2_EN_AP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [software_intr_2_en_ap::W](W) writer structure"]
impl crate::Writable for SOFTWARE_INTR_2_EN_AP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTWARE_INTR_2_EN_AP to value 0"]
impl crate::Resettable for SOFTWARE_INTR_2_EN_AP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
