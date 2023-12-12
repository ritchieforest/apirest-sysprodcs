-- Active: 1695737047766@@127.0.0.1@1433@PRODUCTS
CREATE PROCEDURE [dbo].[c_proveedor_abmlc_sp]
@jsondata nvarchar(max)
as
SET NOCOUNT ON
BEGIN TRY
   DECLARE 
    @nombre_proveedor varchar(200),
    @operacion varchar(20),
    @id_proveedor int,
    @cuit varchar(50),
    @logo varchar(150);
 SET @operacion=(SELECT JSON_VALUE(@jsondata,'$.operacion'));
    SET @id_proveedor=(SELECT JSON_VALUE(@jsondata,'$.id_proveedor'));
    SET @nombre_proveedor=(SELECT JSON_VALUE(@jsondata,'$.nombre_proveedor'));
    SET @cuit=(SELECT JSON_VALUE(@jsondata,'$.cuit'));
    SET @logo=(SELECT JSON_VALUE(@jsondata,'$.logo'));

if @operacion='A'
begin
    insert into sys_prov_01_proveedor(sysprov01_nombre,sysprov01_cuit,sysprov01_logo)
    values(@nombre_proveedor,@cuit,@logo);
    select cast(SCOPE_IDENTITY() as int), @nombre_proveedor,@cuit,@logo;
end 
else if @operacion='M'
begin
    if @id_proveedor=0 or not exists (
        select 1 from sys_prov_01_proveedor where id_sysprov01=@id_proveedor
    )
    begin
        RAISERROR (15600, -1, -1, 'No existe el elemento que desea modificar');
    end 

    if EXISTS (
        select 1 from sys_prov_01_proveedor where sysprov01_cuit=@cuit
        and id_sysprov01!=@id_proveedor
    )
    begin
        RAISERROR (15600, -1, -1, 'Ya existe un proveedor con ese cuit');
    end 

    update sys_prov_01_proveedor
    set 
    sysprov01_nombre=@nombre_proveedor,
    sysprov01_cuit=@cuit,
    sysprov01_logo=logo
    where sysprov01_cuit=@cuit;
end 
else if @operacion='B'
begin
 if @id_proveedor=0 or not exists (
        select 1 from sys_prov_01_proveedor where id_sysprov01=@id_proveedor
    )
    begin
        RAISERROR (15600, -1, -1, 'No existe el elemento que desea modificar');
    end 
    delete from sys_prov_01_proveedor where id_sysprov01=@id_proveedor;
end 
else if @operacion='L'
begin
    SELECT * FROM sys_prov_01_proveedor;
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
        'Error' as TypeResult
END CATCH
SET NOCOUNT OFF;