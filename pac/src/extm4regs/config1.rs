#[doc = "Register `CONFIG1` reader"]
pub struct R(crate::R<CONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRPRI` reader - Indicates what priority interrupt, or base boost, is being used now. CURRPRI represents the preemption priority, and does not indicate secondary priority."]
pub struct CURRPRI_R(crate::FieldReader<u8, u8>);
impl CURRPRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CURRPRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRPRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Branch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRCHSTAT_A {
    #[doc = "0: No hint."]
    NO_HINT = 0,
    #[doc = "1: Conditional branch backwards in decode"]
    COND_BACKWARDS_DECODE = 1,
    #[doc = "2: Conditional branch in decode"]
    COND_DECODE = 2,
    #[doc = "3: Conditional branch in execute"]
    COND_EXECUTE = 3,
    #[doc = "4: Unconditional branch in decode"]
    UNCOND_DECODE = 4,
    #[doc = "5: Unconditional branch in execute"]
    UNCOND_EXECUTE = 5,
    #[doc = "8: Conditional branch in decode taken (cycle after IHTRANS)"]
    COND_DECODE_TAKEN = 8,
}
impl From<BRCHSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: BRCHSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BRCHSTAT` reader - Branch status"]
pub struct BRCHSTAT_R(crate::FieldReader<u8, BRCHSTAT_A>);
impl BRCHSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BRCHSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BRCHSTAT_A> {
        match self.bits {
            0 => Some(BRCHSTAT_A::NO_HINT),
            1 => Some(BRCHSTAT_A::COND_BACKWARDS_DECODE),
            2 => Some(BRCHSTAT_A::COND_DECODE),
            3 => Some(BRCHSTAT_A::COND_EXECUTE),
            4 => Some(BRCHSTAT_A::UNCOND_DECODE),
            5 => Some(BRCHSTAT_A::UNCOND_EXECUTE),
            8 => Some(BRCHSTAT_A::COND_DECODE_TAKEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HINT`"]
    #[inline(always)]
    pub fn is_no_hint(&self) -> bool {
        **self == BRCHSTAT_A::NO_HINT
    }
    #[doc = "Checks if the value of the field is `COND_BACKWARDS_DECODE`"]
    #[inline(always)]
    pub fn is_cond_backwards_decode(&self) -> bool {
        **self == BRCHSTAT_A::COND_BACKWARDS_DECODE
    }
    #[doc = "Checks if the value of the field is `COND_DECODE`"]
    #[inline(always)]
    pub fn is_cond_decode(&self) -> bool {
        **self == BRCHSTAT_A::COND_DECODE
    }
    #[doc = "Checks if the value of the field is `COND_EXECUTE`"]
    #[inline(always)]
    pub fn is_cond_execute(&self) -> bool {
        **self == BRCHSTAT_A::COND_EXECUTE
    }
    #[doc = "Checks if the value of the field is `UNCOND_DECODE`"]
    #[inline(always)]
    pub fn is_uncond_decode(&self) -> bool {
        **self == BRCHSTAT_A::UNCOND_DECODE
    }
    #[doc = "Checks if the value of the field is `UNCOND_EXECUTE`"]
    #[inline(always)]
    pub fn is_uncond_execute(&self) -> bool {
        **self == BRCHSTAT_A::UNCOND_EXECUTE
    }
    #[doc = "Checks if the value of the field is `COND_DECODE_TAKEN`"]
    #[inline(always)]
    pub fn is_cond_decode_taken(&self) -> bool {
        **self == BRCHSTAT_A::COND_DECODE_TAKEN
    }
}
impl core::ops::Deref for BRCHSTAT_R {
    type Target = crate::FieldReader<u8, BRCHSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALTED` reader - In halting mode debug. HALTED remains asserted while the core is in debug."]
pub struct HALTED_R(crate::FieldReader<bool, bool>);
impl HALTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMATTRS` reader - Memory Attributes. \n \n Bit 0 = Allocate \n \n Bit 1 = Shareable."]
pub struct MEMATTRS_R(crate::FieldReader<u8, u8>);
impl MEMATTRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMATTRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMATTRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMATTRD` reader - Memory attributes. Always 0b01 for this bus. They are always Non-shareable, Non-allocate."]
pub struct MEMATTRD_R(crate::FieldReader<u8, u8>);
impl MEMATTRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMATTRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMATTRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMATTRI` reader - Memory attributes. Always 0b01 for this bus. They are always Non-shareable, Non-allocate."]
pub struct MEMATTRI_R(crate::FieldReader<u8, u8>);
impl MEMATTRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMATTRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMATTRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXREQS` reader - Exclusive Request. EXREQS is an address phase control signal that indicates if the access is because of a LDREX or STREX:\n \n 0 = No request \n \n 1 = Exclusive request \n \n You can use EXREQS and EXRESPS to synchronize primitives and semaphores."]
pub struct EXREQS_R(crate::FieldReader<bool, bool>);
impl EXREQS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXREQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXREQS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXREQD` reader - Exclusive Request. EXREQD is an address phase control signal that indicates if the access is because of a LDREX or STREX: \n \n 0 = No request \n \n 1 = Exclusive request. \n \n You can use EXREQD and EXRESPD to synchronize primitives and semaphores."]
pub struct EXREQD_R(crate::FieldReader<bool, bool>);
impl EXREQD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXREQD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXREQD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Indicates what priority interrupt, or base boost, is being used now. CURRPRI represents the preemption priority, and does not indicate secondary priority."]
    #[inline(always)]
    pub fn currpri(&self) -> CURRPRI_R {
        CURRPRI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Branch status"]
    #[inline(always)]
    pub fn brchstat(&self) -> BRCHSTAT_R {
        BRCHSTAT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - In halting mode debug. HALTED remains asserted while the core is in debug."]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Memory Attributes. \n \n Bit 0 = Allocate \n \n Bit 1 = Shareable."]
    #[inline(always)]
    pub fn memattrs(&self) -> MEMATTRS_R {
        MEMATTRS_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Memory attributes. Always 0b01 for this bus. They are always Non-shareable, Non-allocate."]
    #[inline(always)]
    pub fn memattrd(&self) -> MEMATTRD_R {
        MEMATTRD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Memory attributes. Always 0b01 for this bus. They are always Non-shareable, Non-allocate."]
    #[inline(always)]
    pub fn memattri(&self) -> MEMATTRI_R {
        MEMATTRI_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Exclusive Request. EXREQS is an address phase control signal that indicates if the access is because of a LDREX or STREX:\n \n 0 = No request \n \n 1 = Exclusive request \n \n You can use EXREQS and EXRESPS to synchronize primitives and semaphores."]
    #[inline(always)]
    pub fn exreqs(&self) -> EXREQS_R {
        EXREQS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Exclusive Request. EXREQD is an address phase control signal that indicates if the access is because of a LDREX or STREX: \n \n 0 = No request \n \n 1 = Exclusive request. \n \n You can use EXREQD and EXRESPD to synchronize primitives and semaphores."]
    #[inline(always)]
    pub fn exreqd(&self) -> EXREQD_R {
        EXREQD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config1](index.html) module"]
pub struct CONFIG1_SPEC;
impl crate::RegisterSpec for CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config1::R](R) reader structure"]
impl crate::Readable for CONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONFIG1 to value 0"]
impl crate::Resettable for CONFIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
