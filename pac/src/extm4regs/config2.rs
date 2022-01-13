#[doc = "Register `CONFIG2` reader"]
pub struct R(crate::R<CONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG2` writer"]
pub struct W(crate::W<CONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG2_SPEC>;
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
impl From<crate::W<CONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPU_DISABLE` reader - Set to disable the Floating Point Arithmetic Unit"]
pub struct FPU_DISABLE_R(crate::FieldReader<bool, bool>);
impl FPU_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPU_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPU_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_DISABLE` writer - Set to disable the Floating Point Arithmetic Unit"]
pub struct FPU_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_DISABLE_W<'a> {
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
#[doc = "Field `MPU_DISABLE` reader - Set to disable de Memory Protection Unit"]
pub struct MPU_DISABLE_R(crate::FieldReader<bool, bool>);
impl MPU_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPU_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPU_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPU_DISABLE` writer - Set to disable de Memory Protection Unit"]
pub struct MPU_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_DISABLE_W<'a> {
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
#[doc = "Field `DBG_DIS` reader - Set to disable the M4 debugger"]
pub struct DBG_DIS_R(crate::FieldReader<bool, bool>);
impl DBG_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_DIS` writer - Set to disable the M4 debugger"]
pub struct DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_DIS_W<'a> {
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
#[doc = "Field `EDBGEQ` reader - External debug request. Internal use only"]
pub struct EDBGEQ_R(crate::FieldReader<bool, bool>);
impl EDBGEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDBGEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDBGEQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDBGEQ` writer - External debug request. Internal use only"]
pub struct EDBGEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EDBGEQ_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DBGRESTART` reader - Debugger restart. Internal use only"]
pub struct DBGRESTART_R(crate::FieldReader<bool, bool>);
impl DBGRESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGRESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGRESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGRESTART` writer - Debugger restart. Internal use only"]
pub struct DBGRESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRESTART_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Static endianness setting. This signal is sampled at reset, and cannot be changed when reset is inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIGEND_A {
    #[doc = "0: Select little endian as endiannes setting"]
    LITTLE_ENDIAN = 0,
    #[doc = "1: Select big endian as endiannes setting"]
    BIG_ENDIAN = 1,
}
impl From<BIGEND_A> for bool {
    #[inline(always)]
    fn from(variant: BIGEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIGEND` reader - Static endianness setting. This signal is sampled at reset, and cannot be changed when reset is inactive."]
pub struct BIGEND_R(crate::FieldReader<bool, BIGEND_A>);
impl BIGEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIGEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIGEND_A {
        match self.bits {
            false => BIGEND_A::LITTLE_ENDIAN,
            true => BIGEND_A::BIG_ENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN`"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        **self == BIGEND_A::LITTLE_ENDIAN
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN`"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        **self == BIGEND_A::BIG_ENDIAN
    }
}
impl core::ops::Deref for BIGEND_R {
    type Target = crate::FieldReader<bool, BIGEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIGEND` writer - Static endianness setting. This signal is sampled at reset, and cannot be changed when reset is inactive."]
pub struct BIGEND_W<'a> {
    w: &'a mut W,
}
impl<'a> BIGEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIGEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select little endian as endiannes setting"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut W {
        self.variant(BIGEND_A::LITTLE_ENDIAN)
    }
    #[doc = "Select big endian as endiannes setting"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut W {
        self.variant(BIGEND_A::BIG_ENDIAN)
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EXRESPS` reader - Exclusive Response. EXRESPS is a data phase response like HRESPS, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQS and EXRESPS to synchronize primitives and semaphores."]
pub struct EXRESPS_R(crate::FieldReader<bool, bool>);
impl EXRESPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXRESPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXRESPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXRESPS` writer - Exclusive Response. EXRESPS is a data phase response like HRESPS, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQS and EXRESPS to synchronize primitives and semaphores."]
pub struct EXRESPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXRESPS_W<'a> {
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
            (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `EXRESPD` reader - Exclusive Response. EXRESPD is a data phase response like HRESPD, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQD and EXRESPD to synchronize primitives and semaphores."]
pub struct EXRESPD_R(crate::FieldReader<bool, bool>);
impl EXRESPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXRESPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXRESPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXRESPD` writer - Exclusive Response. EXRESPD is a data phase response like HRESPD, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQD and EXRESPD to synchronize primitives and semaphores."]
pub struct EXRESPD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXRESPD_W<'a> {
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
            (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set to disable the Floating Point Arithmetic Unit"]
    #[inline(always)]
    pub fn fpu_disable(&self) -> FPU_DISABLE_R {
        FPU_DISABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to disable de Memory Protection Unit"]
    #[inline(always)]
    pub fn mpu_disable(&self) -> MPU_DISABLE_R {
        MPU_DISABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to disable the M4 debugger"]
    #[inline(always)]
    pub fn dbg_dis(&self) -> DBG_DIS_R {
        DBG_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External debug request. Internal use only"]
    #[inline(always)]
    pub fn edbgeq(&self) -> EDBGEQ_R {
        EDBGEQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debugger restart. Internal use only"]
    #[inline(always)]
    pub fn dbgrestart(&self) -> DBGRESTART_R {
        DBGRESTART_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Static endianness setting. This signal is sampled at reset, and cannot be changed when reset is inactive."]
    #[inline(always)]
    pub fn bigend(&self) -> BIGEND_R {
        BIGEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Exclusive Response. EXRESPS is a data phase response like HRESPS, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQS and EXRESPS to synchronize primitives and semaphores."]
    #[inline(always)]
    pub fn exresps(&self) -> EXRESPS_R {
        EXRESPS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Exclusive Response. EXRESPD is a data phase response like HRESPD, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQD and EXRESPD to synchronize primitives and semaphores."]
    #[inline(always)]
    pub fn exrespd(&self) -> EXRESPD_R {
        EXRESPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to disable the Floating Point Arithmetic Unit"]
    #[inline(always)]
    pub fn fpu_disable(&mut self) -> FPU_DISABLE_W {
        FPU_DISABLE_W { w: self }
    }
    #[doc = "Bit 1 - Set to disable de Memory Protection Unit"]
    #[inline(always)]
    pub fn mpu_disable(&mut self) -> MPU_DISABLE_W {
        MPU_DISABLE_W { w: self }
    }
    #[doc = "Bit 4 - Set to disable the M4 debugger"]
    #[inline(always)]
    pub fn dbg_dis(&mut self) -> DBG_DIS_W {
        DBG_DIS_W { w: self }
    }
    #[doc = "Bit 5 - External debug request. Internal use only"]
    #[inline(always)]
    pub fn edbgeq(&mut self) -> EDBGEQ_W {
        EDBGEQ_W { w: self }
    }
    #[doc = "Bit 6 - Debugger restart. Internal use only"]
    #[inline(always)]
    pub fn dbgrestart(&mut self) -> DBGRESTART_W {
        DBGRESTART_W { w: self }
    }
    #[doc = "Bit 8 - Static endianness setting. This signal is sampled at reset, and cannot be changed when reset is inactive."]
    #[inline(always)]
    pub fn bigend(&mut self) -> BIGEND_W {
        BIGEND_W { w: self }
    }
    #[doc = "Bit 30 - Exclusive Response. EXRESPS is a data phase response like HRESPS, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQS and EXRESPS to synchronize primitives and semaphores."]
    #[inline(always)]
    pub fn exresps(&mut self) -> EXRESPS_W {
        EXRESPS_W { w: self }
    }
    #[doc = "Bit 31 - Exclusive Response. EXRESPD is a data phase response like HRESPD, but is only valid for exclusive accesses and indicates the success or failure of an exclusive operation: \n \n 0 = Exclusive request accepted \n \n 1 = Exclusive request failed. \n \n You can use EXREQD and EXRESPD to synchronize primitives and semaphores."]
    #[inline(always)]
    pub fn exrespd(&mut self) -> EXRESPD_W {
        EXRESPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config2](index.html) module"]
pub struct CONFIG2_SPEC;
impl crate::RegisterSpec for CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config2::R](R) reader structure"]
impl crate::Readable for CONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config2::W](W) writer structure"]
impl crate::Writable for CONFIG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG2 to value 0"]
impl crate::Resettable for CONFIG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
