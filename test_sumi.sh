#!/bin/bash

echo "🚀 Iniciando prueba de sumi..."
echo "------------------------------"

for i in {1..20}
do
    timestamp=$(date +"%T")
    
    if (( i % 3 == 0 )); then
        echo "[$timestamp] ❌ ERROR: Ocurrió un fallo en el paso $i" >&2
        sleep 0.2
        echo "[$timestamp] ⚠️  REINTENTANDO: Intentando recuperar sistema..." >&2
    else
        echo "[$timestamp] ✅ INFO: Procesando tarea número $i..."
        echo "[$timestamp] ⚙️  STATUS: Todo funcionando correctamente."
    fi
    
    sleep 0.5
done

echo "------------------------------"
echo "✅ Prueba finalizada."
