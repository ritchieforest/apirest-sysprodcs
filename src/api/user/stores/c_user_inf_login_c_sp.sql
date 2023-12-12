
CREATE procedure c_user_inf_login_c_sp --'rvillalba','1234'
(
@user varchar(100),
@password varchar(100)
)
as
SET NOCOUNT ON;
BEGIN TRY
select 
suu.sysuser02_active,
suu.sysuser02_usuario,
suu.id_sysuser02,
seem.fk_sysempr01
from sys_user_02_usuarios suu
inner join sys_empl_03_empleados emm on emm.id_sysempl03 =suu.fk_sysempl03
inner join sys_empl_04_empleado_by_empresa seem on seem.fk_sysempl03=suu.fk_sysempl03 and sysempl04_default=1
inner join sys_user_01_tipo_usuarios sut on sut.id_sysuser01 =suu.fk_sysuser01
where 
suu.sysuser02_usuario =@user
and PWDCOMPARE(@password,suu.sysuser02_password)=1;
END TRY 
BEGIN CATCH
    SELECT
        ERROR_NUMBER() AS ErrorNumber,
        ERROR_STATE() AS ErrorState,
        ERROR_SEVERITY() AS ErrorSeverity,
        ERROR_PROCEDURE() AS ErrorProcedure,
        ERROR_LINE() AS ErrorLine,
        ERROR_MESSAGE() AS ErrorMessage
END CATCH
SET NOCOUNT OFF;