#[doc = "Register `SSI_COMP_VERSION` reader"]
pub struct R(crate::R<SSI_COMP_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSI_COMP_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSI_COMP_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSI_COMP_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSI_COMP_VERSION` writer"]
pub struct W(crate::W<SSI_COMP_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSI_COMP_VERSION_SPEC>;
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
impl From<crate::W<SSI_COMP_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSI_COMP_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_COMP_VERSION` reader - Contains the hex representation of the component version. Consists of ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*."]
pub struct SSI_COMP_VERSION_R(crate::FieldReader<u32, u32>);
impl SSI_COMP_VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SSI_COMP_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI_COMP_VERSION_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSI_COMP_VERSION` writer - Contains the hex representation of the component version. Consists of ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*."]
pub struct SSI_COMP_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_COMP_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Contains the hex representation of the component version. Consists of ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*."]
    #[inline(always)]
    pub fn ssi_comp_version(&self) -> SSI_COMP_VERSION_R {
        SSI_COMP_VERSION_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the hex representation of the component version. Consists of ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*."]
    #[inline(always)]
    pub fn ssi_comp_version(&mut self) -> SSI_COMP_VERSION_W {
        SSI_COMP_VERSION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the hex representation of the component version. Consists of ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssi_comp_version](index.html) module"]
pub struct SSI_COMP_VERSION_SPEC;
impl crate::RegisterSpec for SSI_COMP_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssi_comp_version::R](R) reader structure"]
impl crate::Readable for SSI_COMP_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssi_comp_version::W](W) writer structure"]
impl crate::Writable for SSI_COMP_VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSI_COMP_VERSION to value 0x3332_332a"]
impl crate::Resettable for SSI_COMP_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3332_332a
    }
}
