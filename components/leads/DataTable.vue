<script setup lang="ts" generic="TData, TValue">
import type { ColumnDef, SortingState, ColumnFiltersState, VisibilityState, } from '@tanstack/vue-table'

import { valueUpdater } from '@/lib/utils'
import {
    FlexRender,
    getCoreRowModel,
    getPaginationRowModel,
    getFilteredRowModel,
    getSortedRowModel,
    useVueTable,

} from "@tanstack/vue-table"

import {
    DropdownMenu,
    DropdownMenuCheckboxItem,
    DropdownMenuContent,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'


import { ArrowUpDown, ChevronDown } from 'lucide-vue-next'
import { Input } from '@/components/ui/input'


import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"

const props = defineProps<{
    columns: ColumnDef<TData, TValue>[]
    data: TData[]
}>()

const sorting = ref<SortingState>([])
const columnFilters = ref<ColumnFiltersState>([])
const columnVisibility = ref<VisibilityState>({})
const rowSelection = ref({})

const table = useVueTable({
    get data() { return props.data },
    get columns() { return props.columns },
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onSortingChange: updaterOrValue => valueUpdater(updaterOrValue, sorting),
    onColumnFiltersChange: updaterOrValue => valueUpdater(updaterOrValue, columnFilters),
    onColumnVisibilityChange: updaterOrValue => valueUpdater(updaterOrValue, columnVisibility),
    getFilteredRowModel: getFilteredRowModel(),
    onRowSelectionChange: updaterOrValue => valueUpdater(updaterOrValue, rowSelection),
    state: {
        get sorting() { return sorting.value },
        get columnFilters() { return columnFilters.value },
        get columnVisibility() { return columnVisibility.value },
        get rowSelection() { return rowSelection.value },
    },
})

const columnSearchValue = ref('name');
const searchStringPlaceHolder = ref('Search by Name')

const changeColumnSearchId = (id: string) => {
    columnSearchValue.value = id
    if (id === 'name') {
        searchStringPlaceHolder.value = "Search by Name";
    } else if (id === 'website') {
        searchStringPlaceHolder.value = "Search by Website";
    } else if (id === 'phone') {
        searchStringPlaceHolder.value = "Search by Phone Number";
    } else if (id === 'address') {
        searchStringPlaceHolder.value = "Search by Address";
    } else if (id === 'ratings') {
        searchStringPlaceHolder.value = "Search by Ratings";
    } else if (id === 'url') {
        searchStringPlaceHolder.value = "Search by URL";
    } else if (id === 'owner_name') {
        searchStringPlaceHolder.value = "Search by Owner's Name";
    } else if (id === 'location') {
        searchStringPlaceHolder.value = "Search by Location";
    } else if (id === 'status') {
        searchStringPlaceHolder.value = "Search by Status";
    } else if (id === 'actions') {
        searchStringPlaceHolder.value = "Search by Actions";
    } else {
        console.error("Invalid field ID");
    }
}
</script>

<template>
    <div class="flex flex-col h-full">
        <div class="flex items-center py-4 max-h-fit gap-x-2">
            <!--         Change this drop down menus to get to search the drop down that this selects -->
            <Input class="w-full" :placeholder="searchStringPlaceHolder"
                :model-value="table.getColumn(columnSearchValue)?.getFilterValue() as string"
                @update:model-value=" table.getColumn(columnSearchValue)?.setFilterValue($event)" />


            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <Button variant="outline" class="ml-auto">
                        Search Using
                        <ChevronDown class="w-4 h-4 ml-2" />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuCheckboxItem
                        v-for="column in table.getAllColumns().filter((column) => column.getCanHide())"
                        :checked="column.id == columnSearchValue" :key="column.id" class="capitalize"
                        @click="changeColumnSearchId(column.id)">
                        <div> {{ column.id }}

                        </div>

                    </DropdownMenuCheckboxItem>
                </DropdownMenuContent>
            </DropdownMenu>
            <!--         Change this drop down menu above to get to search the drop down that this selects -->
            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <Button variant="outline" class="ml-auto">
                        Columns
                        <ChevronDown class="w-4 h-4 ml-2" />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuCheckboxItem
                        v-for="column in table.getAllColumns().filter((column) => column.getCanHide())" :key="column.id"
                        class="capitalize" :checked="column.getIsVisible()" @update:checked="(value) => {
                            column.toggleVisibility(!!value)
                        }">
                        {{ column.id }}
                    </DropdownMenuCheckboxItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
        <div class="border rounded-md flex-grow h-full overflow-y-auto">
            <Table>
                <TableHeader>
                    <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
                        <TableHead v-for="header in headerGroup.headers" :key="header.id">
                            <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
                                :props="header.getContext()" />
                        </TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody class="h-full overflow-y-auto">
                    <template v-if="table.getRowModel().rows?.length">
                        <TableRow v-for="row in table.getRowModel().rows" :key="row.id"
                            :data-state="row.getIsSelected() ? 'selected' : undefined">
                            <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id">
                                <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
                            </TableCell>
                        </TableRow>
                    </template>
                    <template v-else>
                        <TableRow>
                            <TableCell :colSpan="columns.length" class="h-24 text-center">
                                No results.
                            </TableCell>
                        </TableRow>
                    </template>
                </TableBody>
            </Table>
        </div>
        <div class="w-full flex items-center justify-between">
            <div class="flex-1 text-sm text-muted-foreground">
                {{ table.getFilteredSelectedRowModel().rows.length }} of
                {{ table.getFilteredRowModel().rows.length }} row(s) selected.
            </div>
            <div class="flex items-center justify-end py-4 space-x-2">
                <Button variant="outline" size="sm" :disabled="!table.getCanPreviousPage()"
                    @click="table.previousPage()">
                    Previous
                </Button>
                <Button variant="outline" size="sm" :disabled="!table.getCanNextPage()" @click="table.nextPage()">
                    Next
                </Button>
            </div>
        </div>

    </div>
</template>