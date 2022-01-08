#[doc = "Register `APRebootStatus` reader"]
pub struct R(crate::R<APREBOOTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APREBOOTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APREBOOTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APREBOOTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APRebootStatus` writer"]
pub struct W(crate::W<APREBOOTSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APREBOOTSTATUS_SPEC>;
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
impl From<crate::W<APREBOOTSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APREBOOTSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APRebootStatus` reader - Set to indicate the AP need to reload the code to SRAM HW set to 1 and FW can clear it. This is allowed FW@AP to read this status."]
pub struct APREBOOTSTATUS_R(crate::FieldReader<bool, bool>);
impl APREBOOTSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APREBOOTSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APREBOOTSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APRebootStatus` writer - Set to indicate the AP need to reload the code to SRAM HW set to 1 and FW can clear it. This is allowed FW@AP to read this status."]
pub struct APREBOOTSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> APREBOOTSTATUS_W<'a> {
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
    #[doc = "Bit 0 - Set to indicate the AP need to reload the code to SRAM HW set to 1 and FW can clear it. This is allowed FW@AP to read this status."]
    #[inline(always)]
    pub fn apreboot_status(&self) -> APREBOOTSTATUS_R {
        APREBOOTSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to indicate the AP need to reload the code to SRAM HW set to 1 and FW can clear it. This is allowed FW@AP to read this status."]
    #[inline(always)]
    pub fn apreboot_status(&mut self) -> APREBOOTSTATUS_W {
        APREBOOTSTATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates if AP nees to reload the code to SRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apreboot_status](index.html) module"]
pub struct APREBOOTSTATUS_SPEC;
impl crate::RegisterSpec for APREBOOTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apreboot_status::R](R) reader structure"]
impl crate::Readable for APREBOOTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apreboot_status::W](W) writer structure"]
impl crate::Writable for APREBOOTSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APRebootStatus to value 0"]
impl crate::Resettable for APREBOOTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
