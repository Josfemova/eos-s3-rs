#[doc = "Register `TAMARSTATUS` reader"]
pub struct R(crate::R<TAMARSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMARSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMARSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMARSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Indicate the M4 Power Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum M4POWERSTATUS_A {
    #[doc = "0: Active mode."]
    ACTIVE = 0,
    #[doc = "1: Deep Sleep mode."]
    DEEP_SLEEP = 1,
    #[doc = "2: Shut down mode."]
    SHUT_DOWN = 2,
    #[doc = "3: Clock off mode."]
    CLOCK_OFF = 3,
}
impl From<M4POWERSTATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: M4POWERSTATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `M4PowerStatus` reader - Indicate the M4 Power Status"]
pub struct M4POWERSTATUS_R(crate::FieldReader<u8, M4POWERSTATUS_A>);
impl M4POWERSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M4POWERSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4POWERSTATUS_A {
        match self.bits {
            0 => M4POWERSTATUS_A::ACTIVE,
            1 => M4POWERSTATUS_A::DEEP_SLEEP,
            2 => M4POWERSTATUS_A::SHUT_DOWN,
            3 => M4POWERSTATUS_A::CLOCK_OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == M4POWERSTATUS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        **self == M4POWERSTATUS_A::DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_shut_down(&self) -> bool {
        **self == M4POWERSTATUS_A::SHUT_DOWN
    }
    #[doc = "Checks if the value of the field is `CLOCK_OFF`"]
    #[inline(always)]
    pub fn is_clock_off(&self) -> bool {
        **self == M4POWERSTATUS_A::CLOCK_OFF
    }
}
impl core::ops::Deref for M4POWERSTATUS_R {
    type Target = crate::FieldReader<u8, M4POWERSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates if the M4 Subsystem reset is released (not core reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4RESETSTATUS_A {
    #[doc = "0: M4 Subsystem reset is released."]
    RESET_RELEASED = 0,
    #[doc = "1: M4 Subsystem reset is NOT released."]
    RESET_NOT_RELEASE = 1,
}
impl From<M4RESETSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: M4RESETSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M4ResetStatus` reader - Indicates if the M4 Subsystem reset is released (not core reset)"]
pub struct M4RESETSTATUS_R(crate::FieldReader<bool, M4RESETSTATUS_A>);
impl M4RESETSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4RESETSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4RESETSTATUS_A {
        match self.bits {
            false => M4RESETSTATUS_A::RESET_RELEASED,
            true => M4RESETSTATUS_A::RESET_NOT_RELEASE,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_RELEASED`"]
    #[inline(always)]
    pub fn is_reset_released(&self) -> bool {
        **self == M4RESETSTATUS_A::RESET_RELEASED
    }
    #[doc = "Checks if the value of the field is `RESET_NOT_RELEASE`"]
    #[inline(always)]
    pub fn is_reset_not_release(&self) -> bool {
        **self == M4RESETSTATUS_A::RESET_NOT_RELEASE
    }
}
impl core::ops::Deref for M4RESETSTATUS_R {
    type Target = crate::FieldReader<bool, M4RESETSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This bit is reflect the value on RFUPMU offset 3EC. It will not set for cold start.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4REBOOTREQ_A {
    #[doc = "0: No Code reload is neccesary"]
    NO_RELOAD_NEED = 0,
    #[doc = "1: Need to Re-Load the Code in AP mode"]
    RELOAD_NEEDED = 1,
}
impl From<M4REBOOTREQ_A> for bool {
    #[inline(always)]
    fn from(variant: M4REBOOTREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M4RebootReq` reader - This bit is reflect the value on RFUPMU offset 3EC. It will not set for cold start."]
pub struct M4REBOOTREQ_R(crate::FieldReader<bool, M4REBOOTREQ_A>);
impl M4REBOOTREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4REBOOTREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4REBOOTREQ_A {
        match self.bits {
            false => M4REBOOTREQ_A::NO_RELOAD_NEED,
            true => M4REBOOTREQ_A::RELOAD_NEEDED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RELOAD_NEED`"]
    #[inline(always)]
    pub fn is_no_reload_need(&self) -> bool {
        **self == M4REBOOTREQ_A::NO_RELOAD_NEED
    }
    #[doc = "Checks if the value of the field is `RELOAD_NEEDED`"]
    #[inline(always)]
    pub fn is_reload_needed(&self) -> bool {
        **self == M4REBOOTREQ_A::RELOAD_NEEDED
    }
}
impl core::ops::Deref for M4REBOOTREQ_R {
    type Target = crate::FieldReader<bool, M4REBOOTREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORiniCond` reader - 0xE value if POR trigger, It reflect the value of rfupmu offset 00C"]
pub struct PORINICOND_R(crate::FieldReader<u8, u8>);
impl PORINICOND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PORINICOND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORINICOND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Indicate the M4 Power Status"]
    #[inline(always)]
    pub fn m4power_status(&self) -> M4POWERSTATUS_R {
        M4POWERSTATUS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Indicates if the M4 Subsystem reset is released (not core reset)"]
    #[inline(always)]
    pub fn m4reset_status(&self) -> M4RESETSTATUS_R {
        M4RESETSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is reflect the value on RFUPMU offset 3EC. It will not set for cold start."]
    #[inline(always)]
    pub fn m4reboot_req(&self) -> M4REBOOTREQ_R {
        M4REBOOTREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - 0xE value if POR trigger, It reflect the value of rfupmu offset 00C"]
    #[inline(always)]
    pub fn porini_cond(&self) -> PORINICOND_R {
        PORINICOND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "???\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamarstatus](index.html) module"]
pub struct TAMARSTATUS_SPEC;
impl crate::RegisterSpec for TAMARSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tamarstatus::R](R) reader structure"]
impl crate::Readable for TAMARSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAMARSTATUS to value 0xe0"]
impl crate::Resettable for TAMARSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe0
    }
}
