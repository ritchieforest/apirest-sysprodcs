CREATE  procedure c_configempresas_abmlc_sp --'A','2 Cuota',0
(
	@jsondata nvarchar(max)
)
as
SET NOCOUNT ON;
BEGIN TRY

declare @operation varchar(2),
@id_tarjeta int,
@id_empresa int,
@id_cuota int,
@id_config int,
@porcentaje varchar(15);

 SET @operation=(SELECT JSON_VALUE(@jsondata,'$.operacion'));
 SET @id_tarjeta=cast((SELECT JSON_VALUE(@jsondata,'$.id_tarjeta')) as int);
 SET @id_empresa=cast((SELECT JSON_VALUE(@jsondata,'$.id_empresa')) as int);
 SET @id_cuota=cast((SELECT JSON_VALUE(@jsondata,'$.id_cuota')) as int);
 SET @porcentaje=(SELECT convert(decimal(5,2),JSON_VALUE(@jsondata,'$.porcentaje')));
 SET @id_config=cast((SELECT JSON_VALUE(@jsondata,'$.id_config')) as int);


if @operation='A'
	begin
		if not exists (
			select 1 from sys_targ_03_config_empresas stce2 where 
			fk_systarg01 =@id_tarjeta
			and fk_systarg02 =@id_cuota
			and fk_sysempr01 =@id_empresa
		)
		BEGIN 
			insert into sys_targ_03_config_empresas(fk_systarg01,fk_systarg02,fk_sysempr01,systarg03_porcentaje)
			values(@id_tarjeta,@id_cuota,@id_empresa,@porcentaje);
			select cast(SCOPE_IDENTITY() as int) id_config, @porcentaje porcentaje,@id_cuota id_cuota,@id_tarjeta id_tarjeta,'Exito' TipeResult;	
		END
		else
			begin
				THROW 50001, 'Ya existe un registro similar de esa configuracion', 1;
			end
	end
else if @operation='M'  
	begin
		if exists (
			select 1 from sys_targ_03_config_empresas stce2 where 
			fk_systarg01 =@id_tarjeta
			and fk_systarg02 =@id_cuota
			and fk_sysempr01 =@id_empresa
			and id_systarg03 !=@id_config
		)
		BEGIN 
			THROW 50001, 'Ya existe un registro similar. No se puede modificar', 1;
		END
		
		if EXISTS(select 1 from sys_targ_03_config_empresas  where id_systarg03 =@id_config)
		BEGIN 
			update sys_targ_03_config_empresas 
			SET 
				fk_systarg01=@id_tarjeta,
				fk_systarg02=@id_cuota,
				fk_sysempr01=@id_empresa,
				systarg03_porcentaje =@porcentaje
				
			where id_systarg03 =@id_config;
		END
		else
			begin
				THROW 50001, 'No existe el registro a modificar', 1;
			end
		
	
		select @id_config id_config, @porcentaje porcentaje,@id_cuota id_cuota,@id_tarjeta id_tarjeta,'Exito' TipeResult;
	end
else if @operation='L' 
	begin
		select id_systarg03,systarg03_porcentaje,stt.id_systarg01,stt.systarg01_nombre,stc.id_systarg02 ,stc.systarg02_nombre,'Exito' TipeResult  from sys_targ_03_config_empresas stce 
		inner join sys_targ_01_tarjeta stt (nolock) ON stce.fk_systarg01 =stt.id_systarg01 
		inner join sys_targ_02_cuotas stc (nolock) on stc.id_systarg02 =stce.fk_systarg02 
		where stce.fk_sysempr01 =@id_empresa;
	end
else if @operation='B' 
	begin
		if EXISTS(select 1 from sys_targ_03_config_empresas  where id_systarg03 =@id_config)
			BEGIN 
				delete from sys_targ_03_config_empresas  where id_systarg03 =@id_config;
				select @id_config id_config, @porcentaje,'Exito' TipeResult;
			END
		else
		begin
			THROW 50001, 'EL dato que desea eliminar no existe', 1;
		end
	end		
	
END TRY 
BEGIN CATCH
    SELECT
    	
        ERROR_NUMBER() AS ErrorNumber,
        ERROR_STATE() AS ErrorState,
        ERROR_SEVERITY() AS ErrorSeverity,
        ERROR_PROCEDURE() AS ErrorProcedure,
        ERROR_LINE() AS ErrorLine,
        ERROR_MESSAGE() AS ErrorMessage,
        'Error' as TipeResult,
        concat('c_configempresas_abmlc_sp ',@jsondata) Store
END CATCH
SET NOCOUNT OFF;