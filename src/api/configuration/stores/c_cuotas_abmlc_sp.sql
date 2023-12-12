
CREATE  procedure c_cuotas_abmlc_sp --'A','2 Cuota',0
(
@operation varchar(1),
@descripcion varchar(255),
@id_cuota int
)
as
SET NOCOUNT ON;
BEGIN TRY

if @operation='A'
	begin
		insert into sys_targ_02_cuotas(systarg02_nombre)
		values(@descripcion);
		select cast(SCOPE_IDENTITY() as int) id, @descripcion descripcion;
	end
if @operation='M' and EXISTS(select 1 from sys_targ_02_cuotas where id_systarg02=@id_cuota)
	begin
		update sys_targ_02_cuotas  set systarg02_nombre=@descripcion where id_systarg02=@id_cuota
		select @id_cuota id, @descripcion descripcion;
	end
if @operation='B' and EXISTS(select 1 from sys_targ_02_cuotas where id_systarg02=@id_cuota)
	begin
		delete from sys_targ_02_cuotas where id_systarg02=@id_cuota
		select @id_cuota id, '' descripcion;
	end
if @operation='L' 
	begin
		select id_systarg02,systarg02_nombre  from sys_targ_02_cuotas stc;
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