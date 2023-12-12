
CREATE  procedure c_tarjetas_abmlc_sp --'A','2 Cuota',0
(
@operation varchar(1),
@descripcion varchar(255),
@id_tarjeta int
)
as
SET NOCOUNT ON;
BEGIN TRY

if @operation='A'
	begin
		insert into sys_targ_01_tarjeta(systarg01_nombre)
		values(@descripcion);
		select cast(SCOPE_IDENTITY() as int) id, @descripcion descripcion;
	end
if @operation='M' and EXISTS(select 1 from sys_targ_01_tarjeta  where id_systarg01=@id_tarjeta)
	begin
		update sys_targ_01_tarjeta  set systarg01_nombre=@descripcion where id_systarg01=@id_tarjeta
		select @id_tarjeta id, @descripcion descripcion;
	end
if @operation='B' and EXISTS(select 1 from sys_targ_01_tarjeta  where id_systarg01=@id_tarjeta)
	begin
		delete from sys_targ_01_tarjeta  where id_systarg01=@id_tarjeta
		select @id_tarjeta id, '' descripcion;
	end
if @operation='L' 
	begin
		select id_systarg01 ,systarg01_nombre  from sys_targ_01_tarjeta stt;
	end
	
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