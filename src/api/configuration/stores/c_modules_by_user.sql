
CREATE procedure c_modules_by_user --2,1
(
@user int,
@empresa int
)
as
SET NOCOUNT ON;
BEGIN TRY

select 
sudum.sysuser03_alta,
sudum.sysuser03_baja,sudum.sysuser03_modificacion,sudum.sysuser03_listado,see.id_sysempr01 ,sum2.id_sysuser03 ,sum2.sysuser03_descripcion,sum2.sysuser03_url,sum2.sysuser03_icon,isnull(sum2.fk_sysuser03,0) fk_sysuser03  from sys_user_02_usuarios suu 
inner join sys_user_04_det_usuario_modulos  sudum  on suu.id_sysuser02 =sudum.fk_sysuser02 
inner join sys_empr_01_empresas see on see.id_sysempr01 =sudum.fk_sysempr01 
inner join sys_user_03_modulos sum2  on sum2.id_sysuser03=sudum.fk_sysuser03 
where suu.id_sysuser02 =@user
and  see.id_sysempr01 =@empresa
order by sum2.id_sysuser03  asc, sum2.fk_sysuser03  
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